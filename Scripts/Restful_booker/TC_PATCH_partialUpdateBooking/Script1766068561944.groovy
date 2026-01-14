import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

if (GlobalVariable.token == null || GlobalVariable.token.isEmpty()) {
	KeywordUtil.markFailed("ERROR: El token está vacío. Ejecuta primero el TC de login/token.")
}

def data = [
	('bookingId') : GlobalVariable.bookingid, 
	('firstname') : 'PruebaEditPatchNameQA',
	('lastname') : 'PruebaEditPatchLastnameQA'
]

def response = WS.sendRequest(findTestObject('Restful_booker/PATH_partialUpdateBooking', data))
println "[CREATE] BODY: " + response.getResponseBodyContent()
println "[CREATE] STATUS: " + response.getStatusCode()
println "[CREATE] TIME (ms): " + response.getElapsedTime()
println "[CREATE] HEADERS: " + response.getHeaderFields()

WS.verifyResponseStatusCode(response, 200)
assert response.getElapsedTime() < 6000 : "La API está muy lenta: ${response.getElapsedTime()}ms"

def json = new JsonSlurper().parseText(response.getResponseBodyContent())

//============ VALIDACIONES ESTRUCTUA RESPONSE ============
assert json.firstname == data.firstname : "[ERROR] El firstname no coinciden"
assert json.lastname == data.lastname : "[ERROR] El lastname no coinciden"

//============ ALMACENAR Data de forma global ============
GlobalVariable.firstname = json.firstname
GlobalVariable.lastname = json.lastname