
Short-term
==========

* Port hemisphere and disc sampling so we can have per-pixel sample sets
for scattering that have good distribution

* Pre-generate sample sets at startup
  * Generate a base collection of sample sets. For sample root R, image
    width W, and maximum tracing depth D, generate W collections of
    samples, with each collection containing D sets of size R * R.
  * For each row of rendered image, shuffle the indices into the W
    collections randomly and then render that row of the image.
  * Pass the collection for each pixel to the world trace function, then
    also indicate the sample index (for the current pixel) as an index
    into each of the D sample sets.

  So the final sample data structure will be something like

    all_samples = Vec<Vec<Vec<T>>>
                  ^   ^   ^
                  W   D   R*R

  Here T is likely Vector3<f64> since the samples will be for recursive
  tracing. But we may need other such vectors with T being Vector2<f64>
  for square samples for lights, etc.

  Question: how much memory does this end up using?
  Question: how much time does it take to generate that many sample sets
            at startup?

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

* Command-line arguments (control over sampling, etc.)

* BVH/bounding boxes etc

* Emissive material, environment lighting

Long-term
=========

* SDL2 or similar live graphics output backend
* https://crates.io/crates/tobj
