import sys
from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine
from pathlib import Path

from src.iface.ui.bridge.ui_set import SettingsManager

def main():
    app = QGuiApplication(sys.argv)

    ui_handler = SettingsManager()

    engine = QQmlApplicationEngine()
    engine.rootContext().setContextProperty("uiHandler", ui_handler)

    BASE_DIR = Path(__file__).resolve().parent
    qml_file = BASE_DIR / "src" / "iface" / "ui" / "Main.qml"

    engine.load(str(qml_file))

    if not engine.rootObjects():
        print(f"Error loading QML file: {qml_file}")
        sys.exit(-1)
    
    exit_code = app.exec()

    sys.exit(exit_code)

if __name__ == "__main__":
    main()