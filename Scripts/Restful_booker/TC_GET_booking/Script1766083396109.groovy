import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

def response = WS.sendRequest(findTestObject('Restful_booker/GET_booking', 
	[
		('bookingId') : GlobalVariable.bookingid
	]))

println "RESPUESTA GET: " + response.getResponseBodyContent()

def json = new JsonSlurper().parseText(response.getResponseBodyContent())

WS.verifyResponseStatusCode(response, 200)

assert json.firstname == GlobalVariable.firstname : "[ERROR] firstname no coincide"
assert json.lastname == GlobalVariable.lastname : "[ERROR] lastname no coincide"
assert json.totalprice == GlobalVariable.totalprice : "[ERROR] totalprice no coincide"
assert json.depositpaid == GlobalVariable.depositpaid : "[ERROR] depositpaid no coincide"
assert json.bookingdates.checkin == GlobalVariable.checkin : "[ERROR] checkin no coincide"
assert json.bookingdates.checkout == GlobalVariable.checkout : "[ERROR] checkout no coincide"
assert json.additionalneeds == GlobalVariable.additionalneeds : "[ERROR] additionalneeds no coincide"