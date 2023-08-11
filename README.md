# Instrument Interface

Connect to, command, and query intruments such as oscilloscopes. 

Currently only supports USBTMC connections.

Supported interfaces:
- [x] USBTMC
- [ ] VXI-11

Considered interfaces:
- [ ] GPIB (reason not to: it is old)
- [ ] VICP (reason not to: proprietary)
- [ ] LSIB (reason not to: way too proprietary)

Eventually, and depending on my motivation, this project will become a pure Rust VISA driver. However, in my current position this seems to be a pipe dream.