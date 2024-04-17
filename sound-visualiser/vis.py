from PIL import Image, ImageDraw

import wave
import sys
from itertools import batched

def wav_to_normalised_mono(wavfile):
    with wave.open(wavfile, 'rb') as input_audio:
        length = input_audio.getnframes()
        width = input_audio.getsampwidth()
        channel_max = 2**((width*8)-1)
        raw = []

        for i in range(0, length):
            data = input_audio.readframes(1)
            raw.append([int.from_bytes(x, 'little', signed=True) for x in batched(data, width)])

        return [abs(v[0])/channel_max for v in raw]

def chunk_avg(col, numchunks):
    chunksize = len(col)//numchunks
    return [sum(col[i:i+chunksize])/chunksize for i in range(0, len(col), chunksize)][:numchunks]


def to_img(samples, width, height, bg, fg, scale=1.0, padding=0, minheight=1):
    boxwidth = width//len(samples)
    barwidth = boxwidth-padding
    img = Image.new('RGBA', (width, height), bg)
    d = ImageDraw.Draw(img)
    for x, avg in enumerate(samples):
            barheight = max(avg*height*scale, minheight)
            ystart = (height-barheight)/2
            yend = ystart + barheight
            d.rectangle([(x*boxwidth,ystart), (x*boxwidth+barwidth,yend)], fill=fg)
    return img

def visualise(wavfile, imgfile, dimensions, numchunks, scale=1.0, padding=0, minheight=1, bg=(0,0,0,255), fg=(255,255,255,255)):
    mono = wav_to_normalised_mono(wavfile)
    chunks_avg = chunk_avg(mono, numchunks=numchunks)
    im = to_img(chunks_avg, scale=scale, width=dimensions[0], height=dimensions[1], bg=bg, fg=fg, padding=padding, minheight=minheight)
    im.save(imgfile)

def main():
    visualise('sample-lingala.wav', 'out.png', 
              dimensions=(1000,500), 
              numchunks=100, 
              scale=3.5,
              padding=8,
              minheight=10,
              fg=(255,255,255,255),
              bg=(0,0,0,255)
              )

if __name__ == '__main__':
    main()  