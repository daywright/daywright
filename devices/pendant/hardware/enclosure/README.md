# Enclosure

The pendant body and the necklace. Modelled in FreeCAD, with Blender for the
look. Both are already installed on the owner's laptop.

## v1 reality

With the XIAO ESP32-S3 Sense dev board plus a battery, the first printed pendant
will be chunky (roughly 35 mm across and close to 20 mm thick). That is fine for
a prototype you wear yourself. A custom PCB later (see `../pcb`) makes it much
slimmer, and moving to it changes one firmware board-config line, nothing else.

## Things the enclosure has to solve

- An honest capture indicator LED that is visible when worn.
- The tap sensor reachable through the body.
- A camera window and a mic port.
- Weight balance: consider the battery in the clasp at the back of the neck,
  with the pendant in front.
- Room for the hidden induction earpiece loop in the cord (parking lot).

Export printable STL or 3MF here; keep the FreeCAD source as the master.
