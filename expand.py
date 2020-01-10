from PIL import Image

old = Image.open("out.png")

n = 20

img = Image.new('RGBA', (old.width * n, old.height * n))

for idx in range(old.width):
    for jdx in range(old.height):
        a = old.getpixel((idx, jdx))
        
        for i in range(n):
            for j in range(n):
                img.putpixel((idx * n + i, jdx * n + j), a)
                
img.show()
img.save("big.png")
