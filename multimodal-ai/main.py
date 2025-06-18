
import dspy



def main():
    lm = dspy.LM('openai/gpt-4o-mini')
    dspy.configure(lm=lm)

    print(lm("Hello, how are you?")[0])

if __name__ == "__main__":
    main()
