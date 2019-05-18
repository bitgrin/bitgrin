@set /p TXPath=What is the path to your tx.reponse file? 
@bitgrin wallet finalize -i %TXPath%
@set /p UserInput=Press RETURN to exit