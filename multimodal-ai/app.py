from shiny import App, render, ui, reactive
import seaborn as sns
import tempfile
from pathlib import Path
import pandas as pd
import uuid

app_dir = Path(__file__).parent
df = pd.read_csv(app_dir / "penguins.csv")


penguins = ui.layout_sidebar(
    ui.sidebar(
        ui.input_file(
            "file_up",
            "File Upload",
            multiple=False,
            capture="environment",
        ),
        ui.input_select(
            "var", "Select variable", choices=["bill_length_mm", "body_mass_g"]
        ),
        ui.input_switch("species", "Group by species", value=True),
        ui.input_switch("show_rug", "Show Rug", value=True),
    ),
    ui.output_plot("hist"),
    title="Content Pipeline",
)

ai = [
    ui.page_fluid(
        ui.markdown("## AI"),
        ui.card(
            ui.input_text_area("prompt", "Your AI prompt"),
            ui.input_action_button("generate", "Generate", class_="btn-success", width="300px"),
        ),
        ui.layout_columns(
            ui.column(6, ui.output_image("image")),
            ui.column(6, ui.output_ui("video")),
        ),
    )
]

app_ui = ui.page_navbar(  
    ui.nav_panel("Penguins", penguins), 
    ui.nav_panel("AI",*ai),  
    title="AI Generator Playground",  
    id="page",  
)  


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

def image_url(prompt: str, model: str = "dall-e-2", size: str = "256x256"):
    response = client.images.generate(
        response_format="url",
        prompt=prompt, 
        model=model,
        size=size,
        n=1
    )
    return response.data[0].url



def image_to_byte_array(image: PIL.Image.Image) -> bytes:
  # BytesIO is a file-like buffer stored in memory
  imgByteArr = io.BytesIO()
  # image.save expects a file-like as a argument
  image.save(imgByteArr, format=image.format)
  # Turn the BytesIO object back into a bytes object
  imgByteArr = imgByteArr.getvalue()
  return imgByteArr

from shiny.types import ImgData

from src.vid import gen_video

def server(input, output, session):
    @render.image
    @reactive.event(input.generate, ignore_none=True)
    def image():
        # validate input prompt is not empty
        if not input.prompt():
            return None
        
        # Get image from gen_image
        img = gen_image(input.prompt())

        # save image to tmeporary file
        temp_file = tempfile.NamedTemporaryFile(delete=False, suffix=".png")
        img.save(temp_file.name)

        img: ImgData = {"src": temp_file.name}
        return img
    
    @render.ui
    @reactive.event(input.generate, ignore_none=True)
    def video():
        if not input.prompt():
            return None
        # create a unique path in the www directory for the video
        name = f"{uuid.uuid4()}.mp4"
        filepath = app_dir / "www" / name
        gen_video(input.prompt(), path=filepath)

        return ui.tags.video(src=name, controls=True, autoplay=True, loop=True)

    @render.plot
    def hist():
        hue = "species" if input.species() else None
        sns.kdeplot(df, x=input.var(), hue=hue)
        if input.show_rug():
            sns.rugplot(df, x=input.var(), hue=hue, color="black", alpha=0.25)


app = App(app_ui, server, static_assets=app_dir / "www")