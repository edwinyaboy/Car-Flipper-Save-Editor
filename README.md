# Car Flipper Save Editor

A community-made save editor for <a href="https://store.steampowered.com/app/5343400/Car_Flipper/">**Car Flipper**</a>. The application is written in Rust and uses egui for its interface.

> This is an unofficial community project. It is not made by or affiliated with the game's developer or publisher.

## What it can do

- Edit Cash / XP / Level.
- Edit vehicle advertisements stored in a save file.
- Edit vehicle mileage.
- Change the title condition: Clean, Rebuilt, Salvage, Flood, or Lemon buyback.
- Keep the existing seller, advertising board, and vehicle seed when editing an advertisement.

<details>
  <summary>Click to view screenshots</summary>
<img width="607" height="422" alt="{A0305E31-246F-4618-ADB3-FFF03F02B872}" src="https://github.com/user-attachments/assets/b5818f34-2fd3-4d4d-9502-7e51b9d7d570" />

<img width="607" height="422" alt="{7F9785B6-20B9-4CC7-BC91-E21FD5C43FCD}" src="https://github.com/user-attachments/assets/5bbe6272-7a31-4223-a907-0bad39b63750" />

<img width="607" height="422" alt="{46BE8150-245B-4165-A8A8-96448B2D17B4}" src="https://github.com/user-attachments/assets/344bd8c7-45da-44ff-9317-1e385596d180" />
</details>

## A few things to know

### Phantom GT Convertible

It is not possible to spawn the Phantom GT Convertible using the save editor, since it is a paid vehicle.

It can be acquired in the <a href="https://store.steampowered.com/app/5527310/Car_Flipper_Supporter_Pack/">Car Flipper Supporter Pack.</a> HIGHLY RECOMMENDED.

### Pricing

The editor recalculates its displayed price when relevant advertisement values change. The pricing work is based on reverse-engineering of the game's data and behavior, not on an official game API.
Treat the displayed result as the editor's current calculation, not the actual games' value.

### Damage generation

Authentic damage rolling is **not implemented**.

The defect-selection process has been partly recovered, but the cooked defect data cannot yet be decoded with enough certainty to reproduce the game's roll. The editor does not substitute guessed weights or a made-up damage model. When a different vehicle is selected, existing defect data is preserved rather than being replaced with invented damage.

### Editing safely

Save formats are easy to damage with an incorrect write. Keep an untouched copy of your save and test changes on a duplicate first. If something looks wrong in-game, go back to the original copy rather than continuing to edit the damaged file.

## Using the editor

1. Back up your save file. It will happen automatically, but do it again, just incase.
2. Launch Car Flipper Save Editor, it should automatically choose the save file.
3. Make the changes you need.
4. Click Apply Changes.
5. Start Car Flipper and check the result.
6. ???
7. Profit.

## Building from source

A Rust toolchain is required.

```bash
cargo build --release
```

Run the test suite with:

```bash
cargo test
```

Run Clippy with:

```bash
cargo clippy --all-targets
```

## Reporting a problem

When reporting a bug, include:

- The game version.
- The editor version or commit you are using.
- What you changed before the problem appeared.
- What you expected to happen and what happened instead.
- Any error message from the editor.

## Compatibility

Save formats and game data can change between game updates. Compatibility with a particular game version should be confirmed through testing; don't assume that a save from a newer version will work just because an older one does.
