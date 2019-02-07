[![Build Status](https://travis-ci.org/bitgrin/bitgrin.svg?branch=master)](https://travis-ci.org/bitgrin/bitgrin)

# BitGrin

BitGrin is an in-progress implementation of the MimbleWimble protocol. Many characteristics are still undefined but the following constitutes a first set of choices:

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

BitGrin likes itself small and easy on the eyes. It wants to be inclusive and welcoming for all walks of life, without judgement. BitGrin is terribly ambitious, but not at the detriment of others, rather to further us all. It may have strong opinions to stay in line with its objectives, which doesn't mean disrespect of others' ideas.

We believe in pull requests, data and scientific research. We do not believe in unfounded beliefs.

## Credits

Tom Elvis Jedusor for the first formulation of MimbleWimble.

Andrew Poelstra for his related work and improvements.

John Tromp for the Cuckoo Cycle proof of work.

J.K. Rowling for making it despite extraordinary adversity.

## License

Apache License v2.0.
