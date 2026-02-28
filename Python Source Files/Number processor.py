import math
proc = 1
while True:
    num1 = int(input('first number:'))
    num2 = int(input('second number:'))
    process = input('''choose a process:
-add             -sqare root(sqrt)
-subtract        -logarithm(log)
-multiply        -nth root
-divide
-exponential
(To exit, type'Quit')
>''').lower()
    if process == 'quit':
        break
    elif process == 'add':
        print(num1 + num2)
    elif process == 'subtract':
        print(num1-num2)
    elif process == 'multiply':
        print(num1 * num2)
    elif process == 'divide':
        print(num1/num2)
    elif process == 'exponential':
        print(num1, 'to power', num2, 'is', num1**num2,)
    elif process == 'square root':
        print('square root of', num1, ':', num1**(1/2))
        print('square root of', num2, ':', num2**(1/2))
    elif process == 'nth root':
        print(num1, 'to root', num2, 'is', num1**(1/num2))
    elif process == 'logarithm' or 'log':
        print('log', num1, 'to base', num2, 'is', math.log(num1, num2))
    else:
        print('Please select a stated process.')
