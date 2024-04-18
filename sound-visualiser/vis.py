from PIL import Image, ImageDraw

import wave
import sys
from itertools import batched

def wav_to_normalised_mono(input_audio, channel_select=0):
    length = input_audio.getnframes()
    width = input_audio.getsampwidth()
    channel_max = 2**((width*8)-1)
    raw = []

    for i in range(0, length):
        data = input_audio.readframes(1)
        raw.append([int.from_bytes(x, 'little', signed=True) for x in batched(data, width)])

    # Don't like taking the absolute value here, 
    # as it will not be refection of the true envelope, 
    # but it's a quick fix for now.
    return [abs(v[channel_select])/channel_max for v in raw]

def chunk_avg(col, numchunks):
    chunksize = len(col)//numchunks
    return [sum(col[i:i+chunksize])/chunksize for i in range(0, len(col), chunksize)][:numchunks]


def to_img(samples, width, height, bg, fg, scale=1.0, padding=0, minheight=1):
    boxwidth = width//len(samples)
    print(f"width: {width}, boxwidth: {boxwidth}, samples: {len(samples)}, samplespace: {boxwidth*len(samples)}")
    barwidth = boxwidth-padding
    img = Image.new('RGBA', (width, height), bg)
    d = ImageDraw.Draw(img)
    for i, avg in enumerate(samples):
            barheight = max(avg*height*scale, minheight)
            ystart = (height-barheight)/2
            yend = ystart + barheight
            d.rectangle([(i*boxwidth,ystart), (i*boxwidth+barwidth,yend)], fill=fg)
    return img

def visualise(audio, dimensions, numchunks, scale=1.0, padding=0, minheight=1, bg=(0,0,0,255), fg=(255,255,255,255)):
    mono = wav_to_normalised_mono(audio)
    chunks_avg = chunk_avg(mono, numchunks=numchunks)
    im = to_img(chunks_avg, scale=scale, width=dimensions[0], height=dimensions[1], bg=bg, fg=fg, padding=padding, minheight=minheight)
    return im

def main():
    audio = wave.open('assets/sample-lingala.wav', 'rb')

    xy = (3000,2000)

    bg = Image.new('RGBA', xy, (15,15,15,255))

    gradient = Image.open('assets/gradient.jpg').resize(xy)
    
    mask = visualise(audio, 
              dimensions=xy, 
              numchunks=3000//32, 
              scale=5.0,
              padding=24 ,
              minheight=10,
              fg=(0,0,0,255),
              bg=(0,0,0,0)
              )

    im = Image.composite(gradient, bg, mask)

    im.save('out.png')

if __name__ == '__main__':
    main()  