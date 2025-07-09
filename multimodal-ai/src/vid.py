# import torch
# from diffusers import MochiPipeline
# from diffusers.utils import export_to_video

# pipe = MochiPipeline.from_pretrained("genmo/mochi-1-preview")

# # Enable memory savings
# pipe.enable_model_cpu_offload()
# pipe.enable_vae_tiling()

# prompt = "Close-up of a chameleon's eye, with its scaly skin changing color. Ultra high resolution 4k."

# with torch.autocast("cuda", torch.bfloat16, cache_enabled=False):
#       frames = pipe(prompt, num_frames=85).frames[0]

# export_to_video(frames, "mochi.mp4", fps=30)



# import torch
# from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, MochiTransformer3DModel, MochiPipeline
# from diffusers.utils import export_to_video
# from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

# quant_config = BitsAndBytesConfig(load_in_8bit=True)
# text_encoder_8bit = T5EncoderModel.from_pretrained(
#     "genmo/mochi-1-preview",
#     subfolder="text_encoder",
#     quantization_config=quant_config,
#     torch_dtype=torch.float16,
# )

# quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
# transformer_8bit = MochiTransformer3DModel.from_pretrained(
#     "genmo/mochi-1-preview",
#     subfolder="transformer",
#     quantization_config=quant_config,
#     torch_dtype=torch.float16,
# )

# pipeline = MochiPipeline.from_pretrained(
#     "genmo/mochi-1-preview",
#     text_encoder=text_encoder_8bit,
#     transformer=transformer_8bit,
#     torch_dtype=torch.float16,
#     device_map="balanced",
# )

# video = pipeline(
#   "Close-up of a cats eye, with the galaxy reflected in the cats eye. Ultra high resolution 4k.",
#   num_inference_steps=28,
#   guidance_scale=3.5
# ).frames[0]
# export_to_video(video, "cat.mp4")


# import torch
# from diffusers import MochiPipeline
# from diffusers.utils import export_to_video

# pipe = MochiPipeline.from_pretrained("genmo/mochi-1-preview", variant="bf16", torch_dtype=torch.bfloat16)

# # Enable memory savings
# pipe.enable_model_cpu_offload()
# pipe.enable_vae_tiling()

# prompt = "Close-up of a chameleon's eye, with its scaly skin changing color. Ultra high resolution 4k."
# frames = pipe(prompt, num_frames=85).frames[0]

# export_to_video(frames, "mochi.mp4", fps=30)

# import torch
# from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, AllegroTransformer3DModel, AllegroPipeline
# from diffusers.utils import export_to_video
# from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

# quant_config = BitsAndBytesConfig(load_in_8bit=True)
# text_encoder_8bit = T5EncoderModel.from_pretrained(
#     "rhymes-ai/Allegro",
#     subfolder="text_encoder",
#     quantization_config=quant_config,
#     torch_dtype=torch.float16,
# )

# quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
# transformer_8bit = AllegroTransformer3DModel.from_pretrained(
#     "rhymes-ai/Allegro",
#     subfolder="transformer",
#     quantization_config=quant_config,
#     torch_dtype=torch.float16,
# )

# pipeline = AllegroPipeline.from_pretrained(
#     "rhymes-ai/Allegro",
#     text_encoder=text_encoder_8bit,
#     transformer=transformer_8bit,
#     torch_dtype=torch.float16,
#     device_map="balanced",
# )

# prompt = (
#     "A seaside harbor with bright sunlight and sparkling seawater, with many boats in the water. From an aerial view, "
#     "the boats vary in size and color, some moving and some stationary. Fishing boats in the water suggest that this "
#     "location might be a popular spot for docking fishing boats."
# )
# video = pipeline(prompt, guidance_scale=7.5, max_sequence_length=512).frames[0]
# export_to_video(video, "harbor.mp4", fps=15)


# from diffusers import DiffusionPipeline

# pipe = DiffusionPipeline.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5")
# pipe = pipe.to("mps")

# # Recommended if your computer has < 64 GB of RAM
# pipe.enable_attention_slicing()

# prompt = "a photo of an astronaut riding a horse on mars"
# image = pipe(prompt).images[0]
# image.save("astronaut_horse.png")



# from diffusers import DiffusionPipeline
# import torch

# pipeline = DiffusionPipeline.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5", torch_dtype=torch.float16)
# pipeline.to("mps")
# i = pipeline("An image of a squirrel in Picasso style").images[0]
# i.save("squirrel.png")

import torch
from diffusers import DiffusionPipeline
from diffusers.utils import export_to_video

pipe = DiffusionPipeline.from_pretrained("damo-vilab/text-to-video-ms-1.7b", torch_dtype=torch.float16, variant="fp16")
pipe = pipe.to("mps")


def gen_video(prompt: str, path: str = None):
    video_frames = pipe(prompt).frames[0]
    video_path = export_to_video(video_frames, output_video_path=path)
    return video_path


if __name__ == "__main__":
    print(gen_video("Spiderman is surfing", "spiderman.mp4"))

