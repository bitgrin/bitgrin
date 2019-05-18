@set /p IPAddress=What IP address would you like to send to? 
@bitgrin wallet send -d http://%IPAddress%:8515 amount
@set /p UserInput=Press RETURN to exit