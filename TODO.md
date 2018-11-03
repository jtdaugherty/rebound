
Short-term
==========

* Improve camera rendering method abstraction to avoid the
pinhole/thin-lens code duplication

* Add timing functionality to report on the duration of various
operations. We want to know how much time it takes to:
  * Generate samples
  * Render
  * Later: set up acceleration structures, load and prepare meshes, etc.

* Light power settings (look at how the ground up book does this and see
how we can adapt it)

* Need output machinery
  * Image accumulator thread
  * Worker threads work on regions of the image and send chunks to
    accumulator

* Test lambertian and metal scattering with hemisphere sampling rather
than spherical sampling with rejection, compare results at high sample
counts

* BVH/bounding boxes etc

* Emissive material, environment lighting

* Reorganize source tree to move most things to a central library crate,
then make the current main program use that crate and add new binaries
like sampling helper tools and eventually a network rendering helper

* Scene disk file format?

* Write some rustdocs

Long-term
=========

* SDL2 or similar live graphics output backend
* https://crates.io/crates/tobj
