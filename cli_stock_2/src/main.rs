use clap::{Parser};

use std::io::{
    Error,
    ErrorKind,
};

use chrono::prelude::*;

use yahoo_finance_api as yahoo;

use async_std::prelude::*;
use async_trait::async_trait;

#[derive(Parser, Debug)]
#[clap(version = "1.2",
       author = "Pavel SRB <prace@srbpavel.cz>",
       about = "A Manning LiveProject: async Rust -> created by Claus Matzinger",
)]
struct Opts {
    /// symbols as string like AAPL,MSFT,UBER or filename
    #[clap(short,
           long,
           default_value = "AAPL,MSFT,UBER",
    )]
    symbols: String,

    /// datetime format 2020-12-20T00:00:00Z
    #[clap(short,
           long,
    )]
    from: String,

    /// display debug info
    #[clap(parse(try_from_str))]
    #[clap(short,
           long,
           default_value = "false",
    )]
    verify: bool,
}


type RecordType = f64;

/// computed values for each collect member
#[derive(Debug)]
struct Record {
    min: RecordType,
    max: RecordType,
    last: RecordType,
    diff: RecordType,
    sma: Vec<RecordType>,
}


/// input values
struct Signal {
    window_size: usize
}

#[allow(dead_code)]
impl Signal {
    pub fn default() -> Signal {
        Signal {
            window_size: 1
        }
    }
}

#[async_trait]
trait AsyncStockSignal {
    type SignalType;

    async fn min(&self, series: &[f64]) -> Option<Self::SignalType>;
    async fn max(&self, series: &[f64]) -> Option<Self::SignalType>;
    async fn last(&self, series: &[f64]) -> Option<Self::SignalType>;
}

/// for simple f64 result's
#[async_trait]
impl AsyncStockSignal for Signal {
    type SignalType = f64;

    async fn min(&self,
                 series: &[f64]) -> Option<Self::SignalType> {

        if series.is_empty() {
            None
        } else {
            Some(series.iter().fold(f64::MAX, |acc, q| acc.min(*q)))
        }
    }
            
    async fn max(&self,
                 series: &[f64]) -> Option<Self::SignalType> {

        if series.is_empty() {
            None
        } else {
            Some(series.iter().fold(f64::MIN, |acc, q| acc.max(*q)))
        }
    }

    async fn last(&self,
                  series: &[f64]) -> Option<Self::SignalType> {

        match series.last() {
            Some(l) => Some(*l),
            None => None,
        }
    }
}

#[async_trait]
trait PriceDifference {
    type SignalType;

    async fn diff(&self,
                  series: &[f64]) -> Option<Self::SignalType>;
}

/// for tuple (f64, f64) <- (ABS, REL)
/// no need to shrink it to single f64 in case of future use
#[async_trait]
impl PriceDifference for Signal {
    type SignalType = (f64, f64);

    async fn diff(&self,
                  series: &[f64]) -> Option<Self::SignalType> {

        if !series.is_empty() {
            // unwrap is safe here even if first == last
            let (first, last) = (series.first().unwrap(), series.last().unwrap());
            let abs_diff = last - first;
            let first = if *first == 0.0 { 1.0 } else { *first };
            let rel_diff = abs_diff / first;

            Some((abs_diff, rel_diff))

        } else {

            None
        }
    }
}

#[async_trait]
trait WindowedSMA {
    type SignalType;

    async fn sma(&self,
                 series: &[f64]) -> Option<Self::SignalType>;
}

/// for Vec<f64>
#[async_trait]
impl WindowedSMA for Signal {
    type SignalType = Vec<f64>;

    async fn sma(&self,
                 series: &[f64]) -> Option<Self::SignalType> {

        if !series.is_empty() && self.window_size > 1 {
            Some(
                series
                    .windows(self.window_size,)
                    .map(|w| w.iter().sum::<f64>() / w.len() as f64)
                    .collect(),
            )

        } else {

            None
        }
    }
}


/// download yahoo stock data
async fn fetch_closing_data(
    symbol: &str,
    beginning: &DateTime<Utc>,
    end: &DateTime<Utc>,
    _debug: bool) -> std::io::Result<Vec<f64>> {

    //println!("FETCH: {} -> {}", symbol, _delay);
    
    let provider = yahoo::YahooConnector::new();

    /*
    pub struct YResponse {
        pub chart: YChart,
    }

    pub struct YChart {
        pub result: Vec<YQuoteBlock>,
        pub error: Option<String>,
    }
    */
    
    let response = provider
        .get_quote_history(symbol, *beginning, *end)
        .await
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;

    /*
    println!("RESPONSE: {:?}",
             response
             .chart
             .result
             .last() //.len()
             .unwrap()
             //.meta
             // .timestamp // 1608561000 len() 10 -> sec
             .indicators // QuoteBlock
             ,
    );

    Result<Vec<Quote>, YahooError>

    pub struct Quote {
        pub timestamp: u64,
        pub open: f64,
        pub high: f64,
        pub low: f64,
        pub volume: u64,
        pub close: f64,
        pub adjclose: f64,
    }

    Quote { timestamp: 1642775400,
            open: 314.80999755859375,
            high: 318.30999755859375,
            low: 303.0400085449219,
            volume: 28661700,
            close: 303.1700134277344,
            adjclose: 303.1700134277344
    }
    */
    
    let mut quotes = response
        .quotes()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;

    /* // DEBUG
    println!("\nLAST: {:?}",
             quotes
             .last()
             .unwrap()
             //.adjclose // this is what we get for our Result Vec<f64>
             ,
    );
    */
    
    if !quotes.is_empty() {
        quotes
            // sort slice with key extraction
            .sort_by_cached_key(|k| k.timestamp);

        /*
        println!("QUOTES SORTED: {:?}",
                 &quotes
                 .last()
                 .unwrap(),
        );
        */

        /* 
        Ok(quotes
           .iter()
           .map(|q| q.adjclose as f64).collect())
         */
        
        let adj = quotes
            .iter()
            // filter adjclose + type conversion
            .map(|q| q.adjclose as f64)
            .collect::<Vec<_>>();

        /*
        println!("QUOTES MAP: {:?}",
                 &adj
                 .last(),
        );
        */
        
        Ok(adj)
            
    } else {

        Ok(vec![])
    }
}

/// computed all value and store in Record
/// Signal traits are ASYNC
async fn a_blocks(closes: &Vec<f64>,
                  _debug: bool) -> Option<Record> {

    /* previous:

    let sma = async {

        if test {
            verify_delay(2,
                         &format!(" {}", "sma"),
                         test).await;
        }
            
        signal.sma(&closes).unwrap_or_default()
    };

    futures::join!(max, min, last, diff, sma);

    https://docs.rs/futures/latest/futures/macro.join.html
    */
    
    if !closes.is_empty() {

        let signal = &Signal { window_size: 30 };

        let max = signal.max(&closes).await.unwrap();
        let min = signal.min(&closes).await.unwrap();
        
        let last = match signal.last(&closes).await {
            Some(l) => l,
            None => 0.0,
        };
        
        let diff = signal.diff(&closes).await.unwrap_or((0.0, 0.0));
        
        let sma = signal.sma(&closes).await.unwrap_or_default();

        let record = Record { max: max,
                              min: min,
                              last: last,
                              diff: diff.1, // (ABS, REL)
                              sma: sma,
        };

        Some(record)
            
    } else {
        
        None
    }
}

/// show computed values is CSV format
fn display_record(record_block: &Option<Record>,
                  symbol: &str,
                  from: &DateTime<Utc>,
                  tick_counter: &u64,
                  time_stamp: DateTime<Utc>,
                  verify_flag: bool) {
    
    match record_block {

        Some(record) => {

            let debug_info = if verify_flag {
                format!("{time_stamp:50} {tick_counter:10}")
            } else {
                String::from("")
            };
            
            println!(
                "{csv:100}{debug}",
                     
                csv=format!(
                    "{f},{symbol},${last:.2},{diff:.2}%,${min:.2},${max:.2},${sma:.2} ",
                    f=from.to_rfc3339(),
                    last=record.last,
                    diff=record.diff * 100.0,
                    min=record.min,
                    max=record.max,
                    sma=record.sma.last().unwrap_or(&0.0),
                ),

                debug=debug_info,
            );
            
        },
        
        None => {},
    }
}

/// command arg symbols as string or filename
/// we exit if path exist but read permission denied
fn choose_symbols(symbols: &str,
                  verify_flag: bool) -> String {

    let symbols_path = std::path::Path::new(symbols);
    
    let symbols_text = match symbols_path.exists() {

        true => {

            let content = match std::fs::read_to_string(symbols_path) {

                Ok(data) => String::from(data.trim()),

                Err(error) => {

                    eprintln!("ERROR: in reading file: {:?}\n>>> REASON: {}",
                              symbols_path,
                              error,
                    );

                    // we do not want to fetch default arg symbols
                    std::process::exit(1)
                },
            };

            if verify_flag {
                println!("FILE_SYMBOLS: <{}>", content);
            }

            content
        },

        false => {

            if verify_flag {
                println!("ARG_SYMBOLS: <{}>", symbols);
            }

            String::from(symbols)
        }
    };

    symbols_text
}

/// download new symbol data and compute values
async fn parse_symbol(symbol: &str,
                      from: &DateTime<Utc>,
                      to: &DateTime<Utc>,
                      tick_counter: &u64,
                      debug: bool) -> Option<Record> {

    // DATA download
    let data = fetch_closing_data(&symbol,
                                  &from,
                                  &to,
                                  debug,
    ).await;

    // Record
    match data {
        
        Ok(closes) => {

            let record_block = a_blocks(&closes,
                                        debug,
            ).await;

            // INTERVAT ts
            let time_stamp = Utc::now();
            
            // display line CSV format
            display_record(&record_block, // -> Option not Future
                           &symbol,
                           &from,
                           &tick_counter,
                           time_stamp,
                           debug);//.await;
            
            record_block
            
        },
        
        Err(why) => {
            eprintln!("ERROR: in FETCH CLOSES for SYMBOL: <{}>\n>>> REASON: {}",
                      symbol,
                      why,
            );

            None
        },
    }
}


#[async_std::main]
async fn main() {

    let opts = Opts::parse();

    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");

    let verify_flag: bool = opts.verify;

    let to = Utc::now(); // for SYMBOLS time window

    let mut start = String::from("");

    if verify_flag {
        start = String::from(format!("\nSTART: {}\n", to));
    }

    let symbols = choose_symbols(&opts.symbols,
                                 verify_flag);

    let all_symbols = symbols
        .split(',')
        .collect::<Vec<_>>();
    
    println!("{}period start,symbol,price,change %,min,max,30d avg",
             start,
    );
    
    let delay = 30;
    //let delay = 5;
    
    let mut interval = async_std::stream::interval(
        std::time::Duration::from_millis(delay*1000));
    
    let mut tick_counter:u64 = 0;

    while let Some(_) = interval.next().await {

        tick_counter += 1;

        if verify_flag {
            println!("\nINTERVAL: {} / {}",
                     tick_counter,
                     Utc::now(),
            );
        }
        
        let queries: Vec<_> = all_symbols
            .iter()
            
            // filter invalid SYMBOL
            //.filter(|&s| !["", "\n", "\r\n"].contains(s))
            .filter(|&s| !"".eq(*s))
            
            .map(|&symbol| parse_symbol(&symbol,
                                        &from,
                                        &to,
                                        &tick_counter,
                                        verify_flag,
            ))
            
            .collect();

        // JOIN all symbols FUTURE
        /* 
        https://docs.rs/futures/latest/futures/future/fn.join_all.html
        
        collection of the outputs of future
        
         */
        let _ = futures::future::join_all(queries).await;
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[async_std::test]
    async fn test_PriceDifference_calculate() {
        let signal = &Signal {..Signal::default()};
        
        assert_eq!(signal.diff(&[]).await,
                   None);

        assert_eq!(signal.diff(&[1.0]).await,
                   Some((0.0, 0.0)));

        assert_eq!(signal.diff(&[1.0, 0.0]).await,
                   Some((-1.0, -1.0)));

        assert_eq!(
            signal.diff(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]).await,
            Some((8.0, 4.0))
        );

        assert_eq!(
            signal.diff(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
            Some((1.0, 1.0))
        );
    }

    #[async_std::test]
    async fn test_MinPrice_calculate() {
        let signal = &Signal {..Signal::default()};

        assert_eq!(signal.min(&[]).await,
                   None);

        assert_eq!(signal.min(&[1.0]).await,
                   Some(1.0));

        assert_eq!(signal.min(&[1.0, 0.0]).await,
                   Some(0.0));

        assert_eq!(
            signal.min(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]).await,
            Some(1.0)
        );

        assert_eq!(
            signal.min(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
            Some(0.0)
        );
    }

    #[async_std::test]
    async fn test_MaxPrice_calculate() {
        let signal = &Signal {..Signal::default()};
        
        assert_eq!(signal.max(&[]).await,
                   None);

        assert_eq!(signal.max(&[1.0]).await,
                   Some(1.0));

        assert_eq!(signal.max(&[1.0, 0.0]).await,
                   Some(1.0));

        assert_eq!(
            signal.max(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]).await,
            Some(10.0)
        );

        assert_eq!(
            signal.max(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
            Some(6.0)
        );
    }

    #[async_std::test]
    async fn test_WindowedSMA_calculate() {
        let series = vec![2.0, 4.5, 5.3, 6.5, 4.7];

        let mut signal = &Signal { window_size: 3 };

        assert_eq!(
            signal.sma(&series).await,
            Some(vec![3.9333333333333336, 5.433333333333334, 5.5])
        );

        signal = &Signal { window_size: 5 };

        assert_eq!(signal.sma(&series).await,
                   Some(vec![4.6]));

        signal = &Signal { window_size: 10 };

        assert_eq!(signal.sma(&series).await,
                   Some(vec![]));
    }

    #[async_std::test]
    async fn test_Last() {
        let signal = &Signal {..Signal::default()};
        
        assert_eq!(signal.last(&[]).await,
                   None);

        assert_eq!(signal.last(&[1.0]).await,
                   Some(1.0));

        assert_eq!(signal.last(&[1.0, 0.0]).await,
                   Some(0.0));

        assert_eq!(
            signal.last(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]).await,
            Some(10.0)
        );

        assert_eq!(
            signal.last(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
            Some(1.0)
        );
    }
}
