import time
import random

words = ['python', 'java', 'c++', 'javascript', 'ruby', 'swift', 'go', 'kotlin', 'typescript', 'rust']
target = random.choice(words)

print(f"次の単語を入力してください：{target}")
start_time = time.time()
answer = input(">> ")
end = time.time()

if answer == target:
    print(f"正解！{end - start_time:.2f}秒")
else:
    print("不正解！")
    print(f"正解は：{target}")
    