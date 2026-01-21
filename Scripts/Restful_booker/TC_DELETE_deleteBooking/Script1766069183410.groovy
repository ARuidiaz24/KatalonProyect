import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

if (GlobalVariable.token == null || GlobalVariable.token.isEmpty()) {
	KeywordUtil.markFailed("ERROR: El token está vacío. Ejecuta primero el TC de login/token.")
}

def response = WS.sendRequest(findTestObject('Restful_booker/DELETE_deleteBooking', 
	[
		('bookingId') : GlobalVariable.bookingid
	]))

println "RESPUESTA DELETE: " + response.getResponseBodyContent()

WS.verifyResponseStatusCode(response, 201)
