use chrono::prelude::*;
use clap::Clap;
use std::io::{Error, ErrorKind};
use yahoo_finance_api as yahoo;

use futures::StreamExt;

use rand::Rng;

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Claus Matzinger",
    about = "A Manning LiveProject: async Rust"
)]
#[derive(Debug)]
struct Opts {
    #[clap(short, long, default_value = "AAPL,MSFT,UBER,GOOG")]
    symbols: String,

    #[clap(short, long)]
    from: String,

    /// DEBUG via sleep to verify async is working
    #[clap(parse(try_from_str))]
    #[clap(short, long, default_value = "false")]
    verify: bool,
}


/// computed values for each collect member
#[derive(Debug)]
struct Record {
    min: f64,
    max: f64,
    last: f64,
    diff: f64,
    sma: Vec<f64>,
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

trait AsyncStockSignal {
    type SignalType;

    fn min(&self, series: &[f64]) -> Option<Self::SignalType>;

    fn max(&self, series: &[f64]) -> Option<Self::SignalType>;

    fn last(&self, series: &[f64]) -> Option<Self::SignalType>;
}

/// for simple f64 result's
impl AsyncStockSignal for Signal {
    type SignalType = f64;

    fn min(&self, series: &[f64]) -> Option<Self::SignalType> {
        min(&series)
    }
            
    fn max(&self, series: &[f64]) -> Option<Self::SignalType> {
        max(&series)
    }

    fn last(&self,
            series: &[f64]) -> Option<Self::SignalType> {

        match series.last() {
            Some(l) => Some(*l),
            None => None,
        }
    }
}

trait PriceDifference {
    type SignalType;

    fn diff(&self, series: &[f64]) -> Option<Self::SignalType>;
}

/// for tuple (f64, f64) <- (ABS, REL)
/// no need to shrink it to single f64 in case of future use
impl PriceDifference for Signal {
    type SignalType = (f64, f64);

    fn diff(&self, series: &[f64]) -> Option<Self::SignalType> {
        price_diff(&series)
    }
}

trait WindowedSMA {
    type SignalType;

    fn sma(&self,
           series: &[f64]) -> Option<Self::SignalType>;
}

/// for Vec<f64>
impl WindowedSMA for Signal {
    type SignalType = Vec<f64>;

    fn sma(&self,
           series: &[f64]) -> Option<Self::SignalType> {
        
        n_window_sma(self.window_size,
                     &series,
        )
    }
}


fn price_diff(a: &[f64]) -> Option<(f64, f64)> {
    if !a.is_empty() {
        // unwrap is safe here even if first == last
        let (first, last) = (a.first().unwrap(), a.last().unwrap());
        let abs_diff = last - first;
        let first = if *first == 0.0 { 1.0 } else { *first };
        let rel_diff = abs_diff / first;
        Some((abs_diff, rel_diff))
    } else {
        None
    }
}

fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if !series.is_empty() && n > 1 {
        Some(
            series
                .windows(n)
                .map(|w| w.iter().sum::<f64>() / w.len() as f64)
                .collect(),
        )
    } else {
        None
    }
}

fn max(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        Some(series.iter().fold(f64::MIN, |acc, q| acc.max(*q)))
    }
}

fn min(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        Some(series.iter().fold(f64::MAX, |acc, q| acc.min(*q)))
    }
}

async fn fetch_closing_data(
    symbol: &str,
    beginning: &DateTime<Utc>,
    end: &DateTime<Utc>,
    _debug: bool,
    _delay: u64) -> std::io::Result<Vec<f64>> {

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

fn _do_sync_symbol(symbol: &str,
                   from: &DateTime<Utc>,
                   to: &DateTime<Utc>,
                   debug: bool,
                   delay: u64) {

    let closes = futures::executor::block_on(
        fetch_closing_data(&symbol,
                           &from,
                           &to,
                           debug,
                           delay,
        )
    );

    parse_closes(&from,
                 &symbol,
                 &closes.unwrap(),
                 debug,
    );
}


async fn _do_async_symbol(symbol: &str,
                          from: &DateTime<Utc>,
                          to: &DateTime<Utc>,
                          debug: bool,
                          delay: u64) {

    let closes = async { 
        fetch_closing_data(&symbol,
                           &from,
                           &to,
                           debug,
                           delay,
        ).await
    };

    //async { // unused
        parse_closes(&from,
                     &symbol,
                     &closes.await.unwrap(),
                     debug,
        );
    //};
}


fn parse_closes(from: &DateTime<Utc>,
                      symbol: &str,
                      closes: &Vec<f64>,
                      debug: bool) {

    if !closes.is_empty() {

        let signal = &Signal { window_size: 30,
        };
        
        let record: Record = futures::executor::block_on(blocks(
            &closes,
            &signal,
            debug,
        ));
        
        // CSV data
        println!(
            "{f},{sy},${l:.2},{p:.2}%,${mi:.2},${ma:.2},${s:.2}",
            f=from.to_rfc3339(),
            sy=symbol,
            l=record.last,
            p=record.diff * 100.0,
            mi=record.min,
            ma=record.max,
            s=record.sma.last().unwrap_or(&0.0)
        );
    }
}

async fn verify_delay(delay: u64,
                      caller: &str,
                      debug: bool) {

    if caller.contains("SYMBOL") {
        println!(">>> SLEEP: start -> {}: {}",
                 caller,
                 delay,
        );
    }
    
    async_std::task::sleep(std::time::Duration::from_millis(delay*100)).await;

    if caller.contains("SYMBOL") {
        println!(">>> SLEEP: end -> {}: {}",
                 caller,
                 delay,
        );
    }
    
    if debug {
        println!("  [{delay}] {caller}");
    }
}

async fn blocks(closes: &Vec<f64>,
                signal: &Signal,
                test: bool) -> Record {

    let max = async {
        // ASYNC verification via SLEEP
        if test {
            verify_delay(5,
                         &format!(" {}", "max"),
                         test).await;

        }

        signal.max(&closes).unwrap()
        
    };

    let min = async {
        if test {
            verify_delay(3,
                         &format!(" {}", "min"),
                         test).await;
        }
        
        signal.min(&closes).unwrap()
    };
    
    let last = async {
        if test {
            verify_delay(1,
                         &format!(" {}", "last"),
                         test).await;
        }
            
        match signal.last(&closes) {
            Some(l) => l,
            None => 0.0,
        }
    };

    let diff = async {
        if test {
            verify_delay(4,
                         &format!(" {}", "diff"),
                         test).await;
        }
            
        signal.diff(&closes).unwrap_or((0.0, 0.0))
    };

    let sma = async {
        if test {
            verify_delay(2,
                         &format!(" {}", "sma"),
                         test).await;
        }
            
        signal.sma(&closes).unwrap_or_default()
    };


    // tuple <- join!()
    let values = futures::join!(max,
                                min,
                                last,
                                diff,
                                sma,
    );

    Record { max: values.0,
             min: values.1,
             last: values.2,
             diff: values.3.1, // (ABS, REL)
             sma: values.4,
    }
}


#[async_std::main]
async fn main() {

    let opts = Opts::parse();
    
    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");
    let to = Utc::now();

    let verify_flag: bool = opts.verify;
    
    println!("period start,symbol,price,change %,min,max,30d avg");

    let all_symbols = opts
        .symbols
        .split(',')
        .collect::<Vec<_>>();
        
    let len = all_symbols.len() as u64;
    println!("LEN: {}", len);

    // ASYNC ITER
    // /*
    futures::stream::iter(all_symbols)
        .for_each_concurrent(20, |symbol| async move {
            
            let mut rng = rand::thread_rng();
            let sleep_ms: u64 = rng.gen_range(0..len*3);

            // SLEEP to verify ASYNC
            if verify_flag {
                
                verify_delay(sleep_ms,
                             &format!("SYMBOL: {}", symbol),
                             verify_flag, // DEBUG
                ).await;
            }
            
            // SYMBOL task
            // /* // SYNC
            _do_sync_symbol(&symbol,
                            &from,
                            &to,
                            verify_flag,
                            sleep_ms,
            );
            // */

            /* // ASYNC
            _do_async_symbol(&symbol,
                             &from,
                             &to,
                             verify_flag,
                             sleep_ms,
            ).await
            */
        }).await
    // */
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn test_PriceDifference_calculate() {
        let signal = &Signal {..Signal::default()};
        
        assert_eq!(signal.diff(&[]), None);

        assert_eq!(signal.diff(&[1.0]), Some((0.0, 0.0)));

        assert_eq!(signal.diff(&[1.0, 0.0]), Some((-1.0, -1.0)));

        assert_eq!(
            signal.diff(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]),
            Some((8.0, 4.0))
        );

        assert_eq!(
            signal.diff(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]),
            Some((1.0, 1.0))
        );
    }


    #[test]
    fn test_MinPrice_calculate() {
        let signal = &Signal {..Signal::default()};

        assert_eq!(signal.min(&[]), None);

        assert_eq!(signal.min(&[1.0]), Some(1.0));

        assert_eq!(signal.min(&[1.0, 0.0]), Some(0.0));

        assert_eq!(
            signal.min(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]),
            Some(1.0)
        );

        assert_eq!(
            signal.min(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]),
            Some(0.0)
        );
    }

    
    #[test]
    fn test_MaxPrice_calculate() {
        let signal = &Signal {..Signal::default()};
        
        assert_eq!(signal.max(&[]), None);

        assert_eq!(signal.max(&[1.0]), Some(1.0));

        assert_eq!(signal.max(&[1.0, 0.0]), Some(1.0));

        assert_eq!(
            signal.max(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]),
            Some(10.0)
        );

        assert_eq!(
            signal.max(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]),
            Some(6.0)
        );
    }

    
    #[test]
    fn test_WindowedSMA_calculate() {
        let series = vec![2.0, 4.5, 5.3, 6.5, 4.7];

        let mut signal = &Signal { window_size: 3 };

        assert_eq!(
            signal.sma(&series),
            Some(vec![3.9333333333333336, 5.433333333333334, 5.5])
        );

        signal = &Signal { window_size: 5 };

        assert_eq!(signal.sma(&series), Some(vec![4.6]));

        signal = &Signal { window_size: 10 };

        assert_eq!(signal.sma(&series), Some(vec![]));
    }

    #[test]
    fn test_Last() {
        let signal = &Signal {..Signal::default()};
        
        assert_eq!(signal.last(&[]), None);

        assert_eq!(signal.last(&[1.0]), Some(1.0));

        assert_eq!(signal.last(&[1.0, 0.0]), Some(0.0));

        assert_eq!(
            signal.last(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]),
            Some(10.0)
        );

        assert_eq!(
            signal.last(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]),
            Some(1.0)
        );
    }
}
