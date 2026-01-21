import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

def response = WS.sendRequest(findTestObject('Restful_booker/PATH_partialUpdateBooking', 
	[
		('bookingId') : GlobalVariable.bookingid, 
		('firstname') : 'Andres', 
		('lastname') : 'Zapata'
	]))

println "RESPUESTA PATH: " + response.getResponseBodyContent()

WS.verifyResponseStatusCode(response, 200)