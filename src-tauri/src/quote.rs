use serde::{Deserialize, Serialize};

/// 单只股票实时行情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockQuote {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub change_percent: f64,
    pub change_amount: f64,
    pub volume: f64,
    pub turnover: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub prev_close: f64,
}

#[derive(Debug, Deserialize)]
struct EastMoneyResponse {
    data: Option<EastMoneyData>,
}

#[derive(Debug, Deserialize)]
struct EastMoneyData {
    diff: Option<Vec<EastMoneyItem>>,
}

#[derive(Debug, Deserialize)]
struct EastMoneyItem {
    f12: Option<String>,
    f14: Option<String>,
    f2: Option<f64>,
    f3: Option<f64>,
    f4: Option<f64>,
    f5: Option<f64>,
    f6: Option<f64>,
    f15: Option<f64>,
    f16: Option<f64>,
    f17: Option<f64>,
    f18: Option<f64>,
}

/// 根据 A 股代码推断东方财富 market 前缀
pub fn market_prefix(code: &str) -> &'static str {
    let first = code.chars().next().unwrap_or('0');
    if first == '6' || first == '5' {
        "1"
    } else {
        "0"
    }
}

/// 根据 A 股代码推断腾讯行情前缀
pub fn tencent_prefix(code: &str) -> &'static str {
    let first = code.chars().next().unwrap_or('0');
    if first == '6' || first == '5' {
        "sh"
    } else if first == '8' || first == '4' {
        "bj"
    } else {
        "sz"
    }
}

/// 构建 secid 列表，如 1.600519,0.000001
pub fn build_secids(codes: &[String]) -> String {
    codes
        .iter()
        .map(|code| format!("{}.{}", market_prefix(code), code))
        .collect::<Vec<_>>()
        .join(",")
}

/// 构建腾讯行情 query，如 sh600519,sz000001
pub fn build_tencent_symbols(codes: &[String]) -> String {
    codes
        .iter()
        .map(|code| format!("{}{}", tencent_prefix(code), code))
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_f64(value: &str) -> f64 {
    value.trim().parse().unwrap_or(0.0)
}

/// 解析腾讯行情单行
fn parse_tencent_line(line: &str, fallback_code: &str) -> Option<StockQuote> {
    let start = line.find('"')? + 1;
    let end = line.rfind('"')?;
    let payload = &line[start..end];
    let parts: Vec<&str> = payload.split('~').collect();
    if parts.len() < 6 {
        return None;
    }

    let code = if parts[2].len() == 6 {
        parts[2].to_string()
    } else {
        fallback_code.to_string()
    };
    let name = parts[1].to_string();
    let price = parse_f64(parts[3]);
    let prev_close = parse_f64(parts[4]);
    let open = parse_f64(parts[5]);
    let high = parts.get(33).map(|v| parse_f64(v)).unwrap_or(price);
    let low = parts.get(34).map(|v| parse_f64(v)).unwrap_or(price);
    let volume = parts.get(6).map(|v| parse_f64(v)).unwrap_or(0.0);
    let turnover = parts.get(37).map(|v| parse_f64(v)).unwrap_or(0.0);
    let change_amount = if prev_close > 0.0 {
        price - prev_close
    } else {
        0.0
    };
    let change_percent = if prev_close > 0.0 {
        (change_amount / prev_close) * 100.0
    } else {
        0.0
    };

    Some(StockQuote {
        code,
        name,
        price,
        change_percent,
        change_amount,
        volume,
        turnover,
        high,
        low,
        open,
        prev_close,
    })
}

/// 从腾讯行情拉取（备用数据源）
async fn fetch_quotes_tencent(codes: &[String]) -> Result<Vec<StockQuote>, String> {
    if codes.is_empty() {
        return Ok(vec![]);
    }

    let symbols = build_tencent_symbols(codes);
    let url = format!("https://qt.gtimg.cn/q={symbols}");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let bytes = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .header("Referer", "https://gu.qq.com/")
        .send()
        .await
        .map_err(|e| format!("请求腾讯行情失败: {e}"))?
        .bytes()
        .await
        .map_err(|e| format!("读取腾讯行情失败: {e}"))?;

    let text = encoding_rs::GBK
        .decode(&bytes)
        .0
        .into_owned();

    let mut quotes = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fallback = codes.get(index).map(String::as_str).unwrap_or("");
        if let Some(quote) = parse_tencent_line(line, fallback) {
            quotes.push(quote);
        }
    }

    if quotes.is_empty() {
        return Err("腾讯行情返回为空".to_string());
    }

    Ok(quotes)
}

/// 从东方财富拉取实时行情
async fn fetch_quotes_eastmoney(codes: &[String]) -> Result<Vec<StockQuote>, String> {
    if codes.is_empty() {
        return Ok(vec![]);
    }

    let secids = build_secids(codes);
    let url = format!(
        "https://push2.eastmoney.com/api/qt/ulist.np/get?fltt=2&fields=f12,f14,f2,f3,f4,f5,f6,f15,f16,f17,f18&secids={secids}"
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .header("Referer", "https://quote.eastmoney.com/")
        .send()
        .await
        .map_err(|e| format!("请求东方财富失败: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("东方财富 HTTP {}", response.status()));
    }

    let body: EastMoneyResponse = response
        .json()
        .await
        .map_err(|e| format!("解析东方财富失败: {e}"))?;

    let items = body
        .data
        .and_then(|d| d.diff)
        .unwrap_or_default();

    let quotes: Vec<StockQuote> = items
        .into_iter()
        .filter_map(|item| {
            Some(StockQuote {
                code: item.f12?,
                name: item.f14.unwrap_or_else(|| "未知".to_string()),
                price: item.f2.unwrap_or(0.0),
                change_percent: item.f3.unwrap_or(0.0),
                change_amount: item.f4.unwrap_or(0.0),
                volume: item.f5.unwrap_or(0.0),
                turnover: item.f6.unwrap_or(0.0),
                high: item.f15.unwrap_or(0.0),
                low: item.f16.unwrap_or(0.0),
                open: item.f17.unwrap_or(0.0),
                prev_close: item.f18.unwrap_or(0.0),
            })
        })
        .collect();

    if quotes.is_empty() {
        return Err("东方财富返回为空".to_string());
    }

    Ok(quotes)
}

/// 拉取实时行情（东方财富优先，失败则降级腾讯）
pub async fn fetch_quotes(codes: &[String]) -> Result<Vec<StockQuote>, String> {
    match fetch_quotes_eastmoney(codes).await {
        Ok(quotes) => Ok(quotes),
        Err(primary_err) => {
            fetch_quotes_tencent(codes)
                .await
                .map_err(|fallback_err| format!("{primary_err}; 备用源: {fallback_err}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_market_prefix() {
        assert_eq!(market_prefix("600519"), "1");
        assert_eq!(market_prefix("000001"), "0");
    }

    #[test]
    fn builds_tencent_symbol() {
        assert_eq!(build_tencent_symbols(&["600519".into(), "000001".into()]), "sh600519,sz000001");
    }
}
