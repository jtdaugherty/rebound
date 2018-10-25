
Short-term
==========

* Hemisphere and disc sampling
  * So we can have per-pixel sample sets for scattering that have good
    distribution

* Light power settings (look at how the ground up book does this and see
how we can adapt it)

* Need output machinery
  * Image accumulator thread
  * Worker threads work on regions of the image and send chunks to
    accumulator

* Test lambertian and metal scattering with hemisphere sampling rather
than spherical sampling with rejection, compare results at high sample
counts

* Command-line arguments (control over sampling, etc.)

* BVH/bounding boxes etc

* Emissive material, environment lighting

Long-term
=========

* SDL2 or similar live graphics output backend
* https://crates.io/crates/tobj
