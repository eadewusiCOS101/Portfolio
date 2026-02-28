import time

cart = []
checkout_price = 0


def purchase_select(purchase, name1, price1, name2, price2, name3, price3, name4, price4):
    if purchase == "1":
        return [name1, price1]
    elif purchase == "2":
        return [name2, price2]
    elif purchase == "3":
        return [name3, price3]
    elif purchase == "4":
        return [name4, price4]
    else:
        print("Invalid selectiont.")


print("Welcome to Cumia Electronics!")
while True:
    time.sleep(1)
    category = input(
        "\nSelect a category:\nPhones\nLaptops\nSmart security devices\nSmart accessories\n(Use keyword 'checkout' to checkout)\n>")
    if category.lower() == "phones":
        print("|PHONES|")
        print("""1. Samsung Galaxy S25 Ultra - $1299.99
2. Google pixel 10 pro XL - $1249.99
3. Samsung Galaxy A17 - $149.99
4. One plus 11 - $699.99
""")
        time.sleep(1)
        purchase = input(
            "Which phone would you like to buy? (Enter the number)\n")
        carted_item_info = purchase_select(purchase, "Samsung Galaxy S25 Ultra", 1299.99,
                                           "Google pixel 10 pro XL", 1249.99, "Samsung Galaxy A17", 149.99, "One plus 11", 699.99)
        name = carted_item_info[0]
        price = carted_item_info[1]
        cart.append(f"{name} - ${price}")
        checkout_price += price
        print(f"Added {name} to cart.")

    elif category.lower() == 'laptops':
        print("|LAPTOPS|")
        time.sleep(1)
        print("1. HP ZBook Studio G9 - $2,499.99\n2. Dell XPS 15 - $1,799.99\n3. Lenovo ThinkPad X1 Carbon Gen 9 - $1,599.99\n4. HP Elitebook X360 1030 G11 -  $1,499.99")
        purchase = input(
            "Which laptop would you like to buy (Use the numbers)\n")
        carted_item_info = purchase_select(purchase, "HP ZBook Studio G9", 2499.99,
                                           "Dell XPS 15", 1799.99, "Lenovo ThinkPad X1 Carbon Gen 9", 1599.99, "HP Elitebook X360 1030 G11", 1499.99)
        name = carted_item_info[0]
        price = carted_item_info[1]
        cart.append(f"{name} - ${price}")
        checkout_price += price
        print(f"Added {name} to cart.")
    elif category.lower() == "smart security devices":
        print("|SECURITIES|")
        time.sleep(1)
        print("1. Beduvely 1080p Magnetic WiFi Mini Camera - $27.99\n2. REOLINK 4K/8MP Security Bullet PoE IP Camera - $179.99\n3. Smart Front Door Lock Set - $89.99\n4. One way privacy windows -  $1,299.99")
        purchase = input(
            "Which security device would you like to buy (Use the numbers)\n")
        carted_item_info = purchase_select(purchase, "Beduvely 1080p Magnetic WiFi Mini Camera", 27.99,
                                           "REOLINK 4K/8MP Security Bullet PoE IP Camera", 179.99, "Smart Front Door Lock Set", 89.99, "One way privacy windows", 1299.99)
        name = carted_item_info[0]
        price = carted_item_info[1]
        cart.append(f"{name} - ${price}")
        checkout_price += price
        print(f"Added {name} to cart.")
    elif category.lower() == "smart accessories":
        print("|SMART ACCESSORIES|")
        time.sleep(1)
        print("1. Wired Over-Ear Headphones with USB C Connector - $15.99\n2. Goal Zero Flip 36 Portable Phone Charger - $184.99\n3. Samsung Galaxy Buds 3 Pro - $244.10\n4. Type C Charger -  $9.99")
        accessory_purchase = input(
            "Which smart accessory would you like to buy (Use the numbers)\n")
        carted_item_info = purchase_select(accessory_purchase, "Wired Over-Ear Headphones with USB C Connector", 15.99,
                                           "Goal Zero Flip 36 Portable Phone Charger", 184.99, "Samsung Galaxy Buds 3 Pro", 244.10, "Type C Charger", 9.99)
        accessory_name = carted_item_info[0]
        accessory_price = carted_item_info[1]
        cart.append(f"{accessory_name} - ${accessory_price}")
        checkout_price += accessory_price
        print(f"Added {accessory_name} to cart.")
    elif category.lower() == "checkout":
        print("Your cart:")
        for item in cart:
            print(f"- {item}")
        print(
            f"\nThe total cost of this purchase session is ${checkout_price:.2f}")
        break
    else:
        print("Invalid category selection.")
