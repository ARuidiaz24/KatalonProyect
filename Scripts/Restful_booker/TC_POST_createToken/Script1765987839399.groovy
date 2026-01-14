import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

def response = WS.sendRequest(findTestObject('Restful_booker/POST_createToken', [
    'username' : GlobalVariable.username,
    'password' : GlobalVariable.password
]))

WS.verifyResponseStatusCode(response, 200)

def json = new JsonSlurper().parseText(response.getResponseBodyContent())
def token = json?.token

assert token != null && token.trim()
GlobalVariable.token = token

println "Token obtenido y almacenado"