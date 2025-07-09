from litellm import image_generation
import io
import PIL.Image
from openai import OpenAI
import base64

client = OpenAI()


def gen_image(prompt: str, model: str = "dall-e-2", size: str = "256x256"):
    response = client.images.generate(
        response_format="b64_json",
        prompt=prompt, 
        model=model,
        size=size,
        n=1
    )

    data = response.data[0].b64_json
    # assume data contains your decoded image
    file_like = io.BytesIO(base64.b64decode(data))
    img = PIL.Image.open(file_like)
    return img
    
img = gen_image("A cute baby sea otter")


