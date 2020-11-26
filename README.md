# IR Remote Receiver

This crate provides a signal decoder for the commonly found 44-button
infra red remote used commonly with LED strips.  When an IR sensor is
connected to a GPIO pin, this crate can decode the IR signals into
which button was pressed.

## How it works

The IR remote sends pulse length encoded signals.  Among other special
pulses, there are a short and long pulse, corresponding to 0 and 1,
allowing the transmission of binary data.

Every button is associated to a 32 bit word.  This library includes
the mapping of binary codes to buttons in the Signal enum.