import logging
from pathlib import Path

class DisplaySetter:
    PATH_POWER = Path("/sys/class/backlight/panel_backlight@1/bl_power")
    PATH_BRIGHT = Path("/sys/class/backlight/panel_backlight@1/brightness")

    @staticmethod
    def set_brightness(on=True, brightness_percent=100):
        """Sets the display brightness and power state.

        Args:
            on (bool): If True, turns the display on; if False, turns it off.
            brightness_percent (int): Brightness level as a percentage (0-100).
        """
        try:
            # Power state (0 = on, 1 = off)
            if DisplaySetter.PATH_POWER.exists():
                DisplaySetter.PATH_POWER.write_text("0" if on else "1")
            else:
                logging.warning(f"Power path not found: {DisplaySetter.PATH_POWER}")

            # Brightness (Value range 0..31)
            if on and DisplaySetter.PATH_BRIGHT.exists():
                p = max(0.0, min(100.0, float(brightness_percent)))
                target_val = int(round((p / 100.0) * 31))
                DisplaySetter.PATH_BRIGHT.write_text(str(target_val))
            elif on:
                logging.warning(f"Brightness path not found: {DisplaySetter.PATH_BRIGHT}")

        except PermissionError:
            logging.error(f"Permission denied writing to sysfs. Run as root or add udev rules.")
        except Exception as e:
            logging.error(f"Error setting display: {e}")