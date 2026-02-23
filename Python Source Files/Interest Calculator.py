def float_input(prompt):
    x = float(input(prompt))
    return x


p = float_input("What is the principal? ")
r = float_input("What is the rate? ")
t = float_input("What is the annual time period? ")

i = p * r * t * 0.01
c = (p * ((1 + (r * 0.01))**t)) - p

print(f"\nSimple Interest = ₦{i:.2f} \nCompound Interest = ₦{c:.2f}")
