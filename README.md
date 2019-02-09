[![Build Status](https://travis-ci.org/bitgrin/bitgrin.svg?branch=master)](https://travis-ci.org/bitgrin/bitgrin)

# BitGrin

BitGrin is an in-progress implementation of the MimbleWimble protocol. Many characteristics are still undefined but the following constitutes a first set of choices:

  * Improved economics from Grin Core
  * Clean and minimal implementation, aiming to stay as such.
  * Follows the MimbleWimble protocol, which provides great anonymity and scaling characteristics.
  * Cuckoo Cycle proof of work.
  * Relatively fast block time (a minute).
  * Decreasing block reward over time.
  * Transaction fees are based on the number of Outputs created/destroyed and total transaction size.
  * Smooth curve for difficulty adjustments.
  

To learn more, read our [introduction to MimbleWimble and BitGrin](doc/intro.md).

## Status

BitGrin is still an infant, much is left to be done and [contributions](CONTRIBUTING.md) are welcome (see below). Check our [mailing list archives](https://lists.launchpad.net/mimblewimble/) for the latest status.

## Contributing

To get involved, read our [contributing docs](CONTRIBUTING.md).

## Getting Started

To learn more about the technology, read our [introduction](doc/intro.md).

To build and try out BitGrin, see the [build docs](doc/build.md).

## Philosophy

At BitGrin we belive in sound economics, total privacy and open source software. 
Bitcoin economics did withstand the test of time continually onboarding new people and increasing in value.

## BitGrin Economy visual representations

https://bitgrin.io/comparision-of-emission/

![alt text](https://i.imgur.com/HWFn4Gi.png)
![alt text](https://i.imgur.com/EWcUd0F.png)


## Credits

Tom Elvis Jedusor for the first formulation of MimbleWimble.

Andrew Poelstra for his related work and improvements.

John Tromp for the Cuckoo Cycle proof of work.

Grin Core team for creating a lightweight MimbleWimble implementation

## License

Apache License v2.0.
