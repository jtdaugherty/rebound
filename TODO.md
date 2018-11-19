
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

* Live preview window
  * Keybindings to change some params and re-render
  * To make this work, we want the render to become an internal service
    * Want to send the renderer commands
      * Set this scene and set up
      * Render
      * Cancel rendering
    * Want the renderer to send us events
      * Setting up (i.e. generating samples)
      * Rendering started
      * Chunk finished
      * Rendering done
    * Want to get a reference to the renderer's image buffer

* Emissive material, environment lighting

* Reorganize source tree to move most things to a central library crate,
then make the current main program use that crate and add new binaries
like sampling helper tools and eventually a network rendering helper

* Scene disk file format?

Long-term
=========

* https://crates.io/crates/tobj
