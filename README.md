# Dictrs
Sometimes we need to use translation in the command line, and this simple tool can let you satisfy your translation needs through ChatGPT.

## Usage
- cargo build --release 
- cp target/release/dictrs /usr/local/bin/
- config your api_key $HOME/.dictrs.yaml
- dictrs --help

Example:
```
dictrs -- hello
+--------------+------+
| Total tokens | 17   |
+--------------+------+
| Result       | 你好 |
+--------------+------+
```


## Config

*cat $HOME/.dictrs.yaml*

```yaml
openai_api_key: ""
openai_api_org: ""
openai_api_endpoint: "https://api.openai.com/v1/completions"
openai_api_request_form:
  model: text-davinci-003
  temperature: 0.3
  max_tokens: 100
  top_p: 1.00
  frequency_penalty: 0.0
  presence_penalty: 0.0
```