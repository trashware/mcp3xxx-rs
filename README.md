# Rust SPI driver for MCP3xxx family of 10-13-Bit A/D converters

[![Build Status](https://travis-ci.org/trashware/mcp3xxx-rs.svg?branch=master)](https://travis-ci.org/trashware/mcp3xxx-rs)
[![crates.io](https://meritbadge.herokuapp.com/mcp3xxx)](https://crates.io/crates/mcp3xxx)

This crate provides a Rust SPI driver for MCP3xxx family of 10-13-Bit A/D converters.
It provides an easy to use high-level API to interact with the A/D converter.

The implementation currently relies on the [rppal library](https://crates.io/crates/rppal) and as such is limited to Raspberry Pi.
It doesn't rely on [embedded-hal](https://crates.io/crates/embedded-hal) as it currently doesn't provide sufficient API for setting up SPI devices.

--------------------------------------------------

The datasheet for MCP3002 can be found [here](https://ww1.microchip.com/downloads/en/devicedoc/21294e.pdf).  
The datasheet for MCP3004 and MCP3008 can be found [here](https://ww1.microchip.com/downloads/en/devicedoc/21295c.pdf).  
The datasheet for MCP3202 can be found [here](https://ww1.microchip.com/downloads/en/devicedoc/21034d.pdf).  
The datasheet for MCP3204 and MCP3208 can be found [here](https://ww1.microchip.com/downloads/en/DeviceDoc/21298c.pdf).  
The datasheet for MCP3302 and MCP3304 can be found [here](https://ww1.microchip.com/downloads/en/DeviceDoc/21697e.pdf).
