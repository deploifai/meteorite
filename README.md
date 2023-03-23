# ☄️ Meteorite
A fast and simple web server to host your Machine Learning model.

<!-- TOC -->
* [☄️ Meteorite](#-meteorite)
  * [Install the pip package](#install-the-pip-package)
  * [Write your server](#write-your-server)
  * [Project status](#project-status)
  * [Contribute to ☄️ Meteorite](#contribute-to--meteorite)
<!-- TOC -->

## Install the pip package

```shell
pip install meteorite
```

## Write your server

```python
import json
import meteorite

app = meteorite.Meteorite()

@app.predict
def predict(data):
    body = data.decode("utf-8")
    """
    Run your model on the input
    """
    return body

app.start()
```

By default, the server starts at port `4000`. The `predict` function will run with GET/POST requests on `/predict`.

You can go to http://localhost:4000/predict on your browser to see the result, or use a REST API client to test the 
endpoint.

## Project status

This project is under active development. We will not recommend you to use this package for critical applications. 
We will welcome all contributions! Please refer to the contributions section for more details.

Some of the features we're still working on:

- [x] Pass POST request String and JSON into the Python function.
- [x] Return String and JSON with the correct content type headers.
- [ ] Graceful error handling (⚠️ Priority).
- [ ] Customise the route and port for the main task.
- [ ] Allow more datatypes for POST request to the model.
- [ ] Create more examples.

## Contribute to ☄️ Meteorite

Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) docs for details.

[Join our Discord channel](https://discord.gg/qXTn7cZzrZ) if you have more questions.
