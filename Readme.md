# rustpy-tools

![AppVeyor](https://img.shields.io/badge/build-passing-brightgreen)

A useful tool Python module implement in rust-lang.  

### Installation
```bash
pip install rustpy-tools
```

### Quick Start

```python
import rustpy_tools
from rustpy_tools import RegexUtil, AES

rustpy_tools.version
# 0.2.1

RegexUtil.is_match(r'^\w+@\w+$', 'xxx@gamil.com')
# True

message = 'hello, world'
key = 'yrbv8qlrGJsfo28C'
iv = 'TcwqrwsShZ2FfUSi'
rustpy_tools.AES.encrypt(message, key, iv)
# GSk0f16bYJm+SUbd49PBkw==

rustpy_tools.AES.decrypt('GSk0f16bYJm+SUbd49PBkw==', key, iv)
# 'hello, world'
```

### Build wheel
```bash
# install rust nightly and set default
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
rustup default nightly

# create virtual environment by virtualenv 
virtualenv venv
source venv/activate

# install build tools and build 
pip install maturin
maturin build
```

### Credit
[PyO3](https://github.com/PyO3)

### License
rustpy-tools is licensed under the BSD 2-Clause license.
