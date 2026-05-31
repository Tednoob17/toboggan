# Use Cases

Toboggan decouples the computer running the presentation from the device controlling it. This unlocks scenarios that are impossible with a traditional setup.

## Two presenters, one stage

You're presenting as a duo. The first speaker is at the podium, the second is on the other side of the stage — or in the audience. Passing the clicker across the room isn't an option.

- The server runs on the PC connected to the projector.
- One presenter uses the PC.
- The other opens the web client on their phone or tablet.
- Both can advance slides from their own device, wherever they are in the room.

No more saying "can you click for me?" or discreetly handing over the remote.

## No remote? Your phone is enough

The room is equipped but there's no clicker, or its battery is dead. Toboggan turns any smartphone into a remote:

- Start the server on the PC connected to the projector.
- Open the web client on your phone.
- Walk the stage — advance, rewind, jump to any slide from your pocket.

No Bluetooth, no dongle, no special hardware.

## Video cable is far from the podium

The HDMI / DisplayPort jack is at the back of the room, but you want to speak from the front. Stuck next to the projector desk? No:

- Set up the PC next to the projector (where the cable is).
- Move to the podium with your phone or tablet.
- Control slides wirelessly from where you're actually speaking.

## Presenting without your own laptop

Laptop forgotten, dead battery, or a computer that refuses to cooperate on the day. No panic:

- Start the Toboggan server on a colleague's PC, a Raspberry Pi, or even a cloud VM.
- Open the web client on any device — phone, tablet, loaner — and present.
- Slides and terminal sessions run on the server, not on your device.

Also useful for embedded demos: you can launch SSH sessions from the server, run live commands, and control everything from your phone.

## Training rooms and workshops

The instructor advances slides while each participant follows on their own screen:

- The server runs on the instructor's machine.
- The instructor controls the pace from their PC.
- Participants open the web client on their own device (phone, tablet, PC) and see the current slide.
- Everyone follows at their own pace without crowding around the instructor's screen.

## Accessibility

A presenter with limited mobility can control the presentation from a comfortable position (sitting in the audience, from a wheelchair-mounted device, etc.) without needing to stand at the projector desk.

## Q&A and live debugging sessions

During questions, the presenter can approach the audience while keeping control of the slides:

- Phone in pocket, advance slides to answer questions.
- Jump back to show a technical detail without returning to the desk.

## Embedded terminals in slides

Toboggan lets you embed live terminal sessions directly in a slide. Useful for:

- Showing a fuzzer running in real time — the terminal updates live.
- Doing a live debug demo without switching windows.
- Letting remote participants interact with the same terminal from their browser.

The combination of "control slides from any device + embedded terminals" makes Toboggan especially well suited for technical conferences, training sessions, and duo presentations.
