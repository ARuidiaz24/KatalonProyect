import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

long timeAccept = 6000

def response = WS.sendRequest(OR.findTestObject('Posts/DELETE_Post', [('postId'): '1']))
WS.verifyResponseStatusCode(response, 200)
assert response.getElapsedTime() <= timeAccept