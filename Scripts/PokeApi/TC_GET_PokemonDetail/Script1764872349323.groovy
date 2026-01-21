import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

// ------------------Variables------------------
String pokemonId = '1024'
Long timeAccept = 6000

// ------------------Envio request------------------
def response = WS.sendRequest(OR.findTestObject('PokeApi/GET_PokemonDetail', ['pokemonId' : pokemonId]))
WS.verifyResponseStatusCode(response, 200)
assert response.getElapsedTime() <= timeAccept

// ------------------Validaciones------------------
def json = new JsonSlurper().parseText(response.getResponseBodyContent())
assert json.forms[0].name == 'terapagos'