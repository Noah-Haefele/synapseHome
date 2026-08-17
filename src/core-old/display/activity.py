from PySide6.QtCore import QObject, QEvent, QTimer

from src.hardware.display import DisplaySetter
from src.core.set.general import SettingsManager

class ActivityFilter(QObject):
    def __init__(self, display_hardware: DisplaySetter, general_settings_manager: SettingsManager):
        super().__init__()

        self._display_hardware = display_hardware
        self._settings_manager = general_settings_manager
        self._display_is_on = True

        self._settings_manager.settingsChanged.connect(self._apply_settings_live)

        self._idle_timer = QTimer()
        self._idle_timer.setSingleShot(True)
        self._idle_timer.timeout.connect(self._go_to_sleep)
        
        self._apply_settings_live()

    def _apply_settings_live(self):
        new_timeout = self._settings_manager.get_display_time() * 1000
        self._idle_timer.setInterval(new_timeout)
        
        if self._idle_timer.isActive():
            self._idle_timer.start()

        if self._display_is_on:
            self._display_hardware.set_brightness(True, self._settings_manager.get_brightness())

    def eventFilter(self, obj, event):
        if event.type() in [QEvent.MouseButtonPress, QEvent.TouchBegin]:
            if not self._display_is_on:
                self._wake_up()
                self._idle_timer.start()
                return True
            else:
                self._idle_timer.start()
        return False

    def _wake_up(self):
        self._display_is_on = True
        self._display_hardware.set_brightness(True, self._settings_manager.get_brightness())

    def _go_to_sleep(self):
        if self._display_is_on:
            self._display_is_on = False
            self._display_hardware.set_brightness(False)