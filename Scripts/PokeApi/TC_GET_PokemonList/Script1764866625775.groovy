import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

// ------------------Variables------------------
long timeAccept = 6000

// ------------------Envio request------------------
def response = WS.sendRequest(OR.findTestObject('PokeApi/GET_PokemonList'))
WS.verifyResponseStatusCode(response, 200)
assert response.getElapsedTime() < timeAccept

// ------------------Validaciones------------------
def json = new JsonSlurper().parseText(response.getResponseBodyContent())
assert json.results.size() == 10
assert json.results*.name.contains('pikachu') == false

println "Pokemon list: ${json.results[0].name}"
println "Tiempo de ejecución: ${response.getElapsedTime()}"