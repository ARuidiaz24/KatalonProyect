import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

// ============ PARAMETROS ============
def data = [
	'firstname': 'Ivan',
	'lastname': 'Villa',
	'totalprice': 15000,
	'depositpaid': true,
	'checkin': '2025-12-17',
	'checkout': '2025-12-18',
	'additionalneeds': 'PruebaQA'
]

// ============ CONSUMO WS ============	
def response = WS.sendRequest(findTestObject('Restful_booker/POST_createBooking', data ))

println "[CREATE] BODY: " + response.getResponseBodyContent()
println "[CREATE] STATUS: " + response.getStatusCode()
println "[CREATE] TIME (ms): " + response.getElapsedTime()
println "[CREATE] HEADERS: " + response.getHeaderFields()

//============ VALIDACIONES HTTP ============
WS.verifyResponseStatusCode(response, 200) 
assert response.getElapsedTime() < 6000 : "La API está muy lenta: ${response.getElapsedTime()}ms"

//============ PARSEO ============
def json = new JsonSlurper().parseText(response.getResponseBodyContent())

//============ VALIDACIONES ESTRUCTUA RESPONSE ============
assert json.booking.firstname == data.firstname : "[ERROR] El firstname no coinciden"
assert json.booking.lastname == data.lastname : "[ERROR] El lastname no coinciden"
assert json.booking.totalprice == data.totalprice : "[ERROR] El totalprice no coinciden"
assert json.booking.depositpaid == data.depositpaid : "[ERROR] El depositpaid no coinciden"
assert json.booking.bookingdates.checkin == data.checkin : "[ERROR] El checkin no coinciden"
assert json.booking.bookingdates.checkout == data.checkout : "[ERROR] El checkout no coinciden"
assert json.booking.additionalneeds == data.additionalneeds : "[ERROR] El additionalneeds no coinciden"

//============ ALMACENAR Data de forma global ============
GlobalVariable.bookingid = json.bookingid
GlobalVariable.firstname = json.booking.firstname
GlobalVariable.lastname = json.booking.lastname
GlobalVariable.totalprice = json.booking.totalprice
GlobalVariable.depositpaid = json.booking.depositpaid
GlobalVariable.checkin = json.booking.bookingdates.checkin
GlobalVariable.checkout = json.booking.bookingdates.checkout
GlobalVariable.additionalneeds = json.booking.additionalneeds