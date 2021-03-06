/* 
 * Swagger Petstore
 *
 * This is a sample server Petstore server.  You can find out more about Swagger at [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/).  For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Amount : some description 

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
  /// some description 
  #[serde(rename = "value")]
  value: f64,
  #[serde(rename = "currency")]
  currency: ::models::Currency
}

impl Amount {
  /// some description 
  pub fn new(value: f64, currency: ::models::Currency) -> Amount {
    Amount {
      value: value,
      currency: currency
    }
  }

  pub fn set_value(&mut self, value: f64) {
    self.value = value;
  }

  pub fn with_value(mut self, value: f64) -> Amount {
    self.value = value;
    self
  }

  pub fn value(&self) -> &f64 {
    &self.value
  }


  pub fn set_currency(&mut self, currency: ::models::Currency) {
    self.currency = currency;
  }

  pub fn with_currency(mut self, currency: ::models::Currency) -> Amount {
    self.currency = currency;
    self
  }

  pub fn currency(&self) -> &::models::Currency {
    &self.currency
  }


}



