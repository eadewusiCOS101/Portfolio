import math


def discriminant(x, y, z):
    return y**2 - 4*x*z


print("Quadratic solver is now online.")
print("Provided that the form of the equation is ax² + bx + c = 0")
a = float(input("Enter a:"))
if a == 0:
    print("Invalid, a cannot be zero")
    exit()

b = float(input("Enter b:"))
c = float(input("Enter c:"))
d = discriminant(a, b, c)
if d >= 0:
    root1 = ((-b) + math.sqrt(d)) / (2 * a)
    root2 = ((-b) - math.sqrt(d)) / (2 * a)
    print("The roots of the equation {}x² + {}x + {} are {:.2f} , {:.2f}"
          .format(a, b, c, root1, root2))
    if abs(root1 - root2) < 1e-9:
        print("Therefore, the root of the equation is {:.2f} twice."
              .format(root1))
else:
    real_part = -b/(2 * a)
    imaginary_part = (math.sqrt(-d)) / (2 * a)
    r1 = "{:.2f} + {:.2f}i".format(real_part, imaginary_part)
    r2 = "{:.2f} - {:.2f}i".format(real_part, imaginary_part)
    print("The roots of the equation {}x² + {}x + {} are {} and {}"
          .format(a, b, c, r1, r2))
