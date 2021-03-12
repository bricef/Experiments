#!/usr/bin/env python2

from PIL import Image
from PIL import ImageDraw
import numpy as np

CREAM="#e9f3fe"
NAVY="#00172f"

FOREGROUND=CREAM
BACKGROUND=NAVY

def ring(im, origin, radius, thickness=2, color=FOREGROUND):
  outer_box = (origin[0]-radius,origin[1]-radius,origin[0]+radius,origin[1]+radius)
  outer_ring = tuple(np.array(outer_box)+np.array([-thickness,-thickness,thickness,thickness]))
  inner_ring = tuple(
    np.array(outer_ring)
    +
    np.array([thickness*2,thickness*2,-thickness*2,-thickness*2]))

  cream_all = Image.new("RGBA", im.size, color)
  mask = Image.new("1", im.size, 1)

  draw = ImageDraw.Draw(mask)
  draw.ellipse(outer_ring, fill=0 )
  draw.ellipse(inner_ring, fill=1 )
  del draw

  return Image.composite(im,cream_all,mask)

def dot(im, xy, radius=10, border=0, color=FOREGROUND, bordercolor=BACKGROUND):
  cream_all = Image.new("RGBA", im.size, color)
  draw=ImageDraw.Draw(im)
  x,y=xy
  draw.ellipse((x-(radius+border),y-(radius+border),x+(radius+border),y+(radius+border)), fill=bordercolor)
  draw.ellipse((x-radius,y-radius,x+radius,y+radius), fill=color)
  del draw
  return im


def circlet(im, origin, radius, thickness=5, num=13, dotrad=15, border=5, fore=FOREGROUND, back=BACKGROUND):

  im = ring(im,origin, radius, thickness=thickness, color=fore)

  angles = [((2*np.pi)/num)*x for x in range(num)]
  coords = [(np.sin(theta)*radius, np.cos(theta)*radius) for theta in angles]
  points = [(x+origin[0], y+origin[1]) for x,y in coords]

  for point in points:
    im = dot(im, point, radius=3*thickness,border=thickness, color=fore, bordercolor=back)
  return im


def scaledring(im, origin, radius, scale=0.4, thickness=5, color=FOREGROUND):
  cream_all = Image.new("RGBA", im.size, color)
  height = radius*0.4
  outer_box = (origin[0]-radius, origin[1]-height, origin[0]+radius, origin[1]+height)
  outer_ring = tuple(np.array(outer_box)+np.array([-thickness,-thickness,thickness,thickness]))
  inner_ring = tuple(
    np.array(outer_ring)
    +
    np.array([thickness*2,thickness*2,-thickness*2, -thickness*2])
    +
    np.array([0,-thickness*scale,0,-thickness*scale ])
    )
  mask = Image.new("1", im.size, 1)

  draw = ImageDraw.Draw(mask)
  draw.ellipse(outer_ring, fill=0 )
  draw.ellipse(inner_ring, fill=1 )
  del draw
  return Image.composite(im,cream_all,mask)


if __name__ == "__main__":
  im = Image.new("RGBA", (1200,800), BACKGROUND)
#  im = scaledring(im, (300,200), 150, scale=1.5, thickness=2, color = FOREGROUND)
  im = circlet(im, (600,400), 300, thickness=10, dotrad=30, border=10, fore = FOREGROUND, back = BACKGROUND)
  im.save("try.png")
