use structopt::StructOpt;
use std::error::Error;
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(StructOpt)]
struct CLI {

  city_name: String,

}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {

   lon: f32,
   lat: f32,

}
#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}
#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Main {

  temp: Option<f32>,
  feels_like: Option<f32>,
  temp_min: Option<f32>,
  temp_max: Option<f32>,
  pressure: Option<i32>,
  humidity: Option<i32>,
  sea_level: Option<i32>,
  grnd_level: Option<i32>,

}
#[derive(Serialize, Deserialize, Debug)]
struct Wind {

  speed: f32,
  deg: i32,

}
#[derive(Serialize, Deserialize, Debug)]
struct Rain {

  lh : Option<f32>,

}
#[derive(Serialize, Deserialize, Debug)]
struct Cloud {

    all : i32,

}
#[derive(Serialize, Deserialize, Debug)]
struct Sys {

   country: String,
   sunrise: i32,
   sunset: i32,



}
#[derive(Serialize, Deserialize, Debug)]
struct Forecast {

  coord: Option<Coord>,
  weather: Option<Weather>,
  base: Option<String>,
  main: Main,
  visibility: Option<i32>,
  wind: Option<Wind>,
  rain: Option<Rain>,
  clouds: Option<Cloud>,
  dt: Option<i32>,
  sys: Option<Sys>,
  timezone: Option<i32>,
  id: Option<i32>,
  name: Option<String>,
  cod: i32,
}


impl Forecast {

  async fn get_forecast(city_name: &String) -> Result <Forecast, Box<dyn Error>> {

    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={city_name}&appid=5339604efcba8fb8f620a40592a5b63d");
    let url = Url::parse(&url)?;

    let resp = reqwest::get(url)
         .await?
         .json::<Forecast>()
         .await?;

      Ok(resp)

  }


}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

 let args = CLI::from_args();

 let response = Forecast::get_forecast(&args.city_name).await;
    match response{
        Ok(response) => {
            match response.weather{
                Some(val) => {
                    println!("Weather Description: {}", val.details.description);
                },

                None => println!("No weather description available"),
            }
            
            match response.main.temp {
                Some(val) => println!("Current Temperature is {} C ", to_celcius(val)),
                None => println!("No data found for current temperature"),
            }

            match response.main.feels_like {

                Some(val) => println!("But it feels like {} C", to_celcius(val)),
                None => println!("No data found for feels_like temperature"),

            }

            match response.main.temp_min {

                Some(val) => println!("The minimum temperature is {}", to_celcius(val)),
                None      => println!("No data available for minimum temperature"),


            }

            match response.main.temp_max {

                Some(val) => println!("The maximum temperature is {}", to_celcius(val)),
                None => println!("No data available for maximum temperature"),

            }
        }
        Err(e) => {
            println!("{}",e);
        }
    }


 Ok(())
}

fn to_celcius(temp: f32) -> f32{

  temp - 273.15


}
