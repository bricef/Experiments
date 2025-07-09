
import dspy
import warnings


def main():
    with warnings.catch_warnings(record=True) as caught_warnings:

        gtp_o4_mini = dspy.LM('openai/gpt-4o-mini')

        dspy.configure(lm=gpt_o4_mini)

        print(lm("Please provide an outline for a game deisgn document.")[0])
    
    print(f"\n {len(caught_warnings)} warnings caught.") 

if __name__ == "__main__":
    main()








