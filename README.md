Project Title: Rust Embedded - Red LED - Light it Up!

Description:

The purpose of this project is to demonstrate basic digital output control using a microcontroller, specifically the BBC micro:bit V2. The project involves flashing and executing an embedded Rust program to control a single red LED connected to a breadboard and powered by the micro:bit via a Kitronik Edge Connector Breakout Board. 

References 
- Rust Discovery Book for micro:bit 
 - https://docs.rust-embedded.org/discovery/microbit/index.html
- Pinout Diagram for micro:bit V2
   - https://tech.microbit.org/hardware/edgeconnector/#pins-and-signals
- Kitronik Edge Connector Breakout Board - Find Datasheet here:
   - https://kitronik.co.uk/products/5601b-edge-connector-breakout-board-for-bbc-microbit-pre-built

references/images
- See these images as well in the github directories!

Project's required hardware:

- 1 Micro:bit V2
- 1 Battery pack for the micro:bit V2
- 2 AA batteries
- 1 USB-A to Micro USB Cable
- 1 Kitronik Edge Connector Breakout Board
- 1 830 point solderless breadboard
- 1 560 Ohm resistor
- 2 M/F 6" Premium Jumper Wires 
    - 1 Black 
    - 1 Red
- 1 Red LED - 5 mm

Assumption:

You've already installed Rust on your local machine - if you haven't, refer to this section of the Rust Discovery Book 

    - https://docs.rust-embedded.org/discovery/microbit/03-setup/index.html

Step 1. - Clone this repository to your local machine on in your terminal window.

git clone https://github.com/CodingInGreen/microbit_v2_red_led.git

Move into microbit_v2_red_led directory in the terminal.

cd microbit_v2_red_led

Step 2. - Review the code in main.rs. You'll see detailed comments on what every line of code does. The main goal we're trying to accomplish with main.rs is setting the micro:bit V2's voltage to high through pin 1 on the Kitronik Edge Connector Breakout Board. Refer to the Pinout Diagram for micro:bit V2 hyperlink above to see how the code translates into physical pins on the Kitronik Edge Connector Breakout Board.

Step 3. - Plug your micro:bit V2 intoto your computer using the USB cable. 

Step 4. - Setup your micro:bit V2 as the target to be flashed.

rustup target add thumbv7em-none-eabihf

If you run into problems here, review the Rust Discovery Book.

Step 5. - Flash the code to your micro:bit V2. Make sure you're in the microbit_v2_red_led directory.

cargo build --features v2 --target thumbv7em-none-eabihf

If you run into problems here, review the Rust Discovery Book.

Step 6. - After flashing is complete, eject the micro:bit V2 from your computer. Remove it from the USB cable and set it aside.

Step 7. Take out your breadboard and lay it on a dry, flat surface. 

    An 830 pin solderless breadboard is a common tool used in electronics for prototyping and testing circuits without the need for soldering.

    830 Pin Layout: This type of breadboard typically has 830 connection points or "tie points." These are arranged in rows and columns, providing a flexible platform for assembling electronic components and circuits.

    Positive and Negative Rails: On either side of the breadboard, there are typically two sets of long horizontal rows known as rails or buses. These are usually marked with "+" (positive) and "-" (negative) signs. They are used to distribute power throughout the breadboard. 

    Central Gap: The breadboard has a central gap or divider running down the middle. This gap is crucial because it prevents short circuits between the legs of integrated circuits (ICs) or other dual-inline components when they are placed over the gap.

    Rows A through J: On either side of the central gap, there are rows on each side. Labeled A through E on one side, and F through J on the other side. These rows contain sets of five connection points each. 

    Connection points in multiples of 5's: The numbers running from 1 to 65 in multiples of 5 on the breadboard's columns are there for reference and organization. They serve a couple of key purposes, but mainly when working with others or following instructions , these numbers allow for clear communication about where components should be placed. 

Step 8. Take out your LED - 5 mm. In electrical and electronic components like diodes and LEDs (Light Emitting Diodes), the terms "anode" and "cathode" refer to the polarity of the terminals:

    - Anode: The anode is the positive (+) terminal of the component. For an LED, the anode is the longer leg and is connected to the positive side of the power source.

    - Cathode: The cathode is the negative (−) terminal. For an LED, the cathode is the shorter leg and is typically connected to the ground or the negative side of the power source.

Step 9. Place the longer leg of the LED (The anode, positive (+) leg) into h21 on the breadboard. Place the shorter leg of the LED (The cathode, negative (-) leg) into h20 on the breadboard.

Step 10. Take out the 560 Ohm Resistor and read the below. 

    A resistor is a fundamental electronic component that resists the flow of electrical current. It's used in circuits to control current flow, divide voltages, and limit the power to other components. Resistors come in various sizes and resistance values, which determine how much they impede the current.

    The resistance value of a resistor is measured in Ohms (Ω), and this value is usually indicated on the resistor using a color code system or numerical markings, especially for surface-mount resistors.

    A 560 Ohm resistor can be used to limit the current going through an LED to prevent it from burning out. The specific value needed depends on the voltage of the power source and the characteristics of the LED.

    With a 3 Volt power source across a 560 Ohm resistor, the current flowing through the resistor would be approximately 0.00536 amperes, or 5.36 milliamperes. This is roughly the amount of current that will be given to the LED in our example.

Step 11. It does not matter which side of the resistor is placed. Insert one pin of the resistor into g21. Insert the other side of the resistor across the gap of the breadboard into c21. 

Step 12. Insert the micro:bit V2's gold pins into the Kitronik Edge Connector Breakout Board. 

Step 13. Take out the 6" M/F Premium Jumper Wire (Black). This is the negative (-) or Ground. Insert the female end into one of the 0V pins on the Kitronik Edge Connector Breakout Board.

Step 14. Take the male end of that same wire, and insert it into pin j20 on the breadboard.

Step 15. Take out the 6" M/F Premium Jumper Wire (Red). This is the positive (+). Insert the female end into pin 1 on the Kitronik Edge Connector Breakout Board.

Step 16. Take the male end of that same wire, and insert it into pin a21 on the breadboard.

Step 17. Insert 2 AA Batteries into the Battery Pack for the micro:bit V2 and close the lid. Connect the Battery Pack to the micro:bit V2.

Step 18. Turn the power switch on the Battery Pack to the on position.

Does your red led light up?








