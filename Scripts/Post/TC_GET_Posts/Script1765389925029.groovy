import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper

def response = WS.sendRequest(OR.findTestObject('Posts/GET_Posts'))
WS.verifyResponseStatusCode(response, 200)
assert response.getElapsedTime() <= (GlobalVariable.slaMs)

def json = new JsonSlurper().parseText(response.getResponseBodyContent())
assert json instanceof List
assert json.size() > 0
KeywordUtil.logInfo("Total posts: ${json.size()}, primeros IDs: " + json)
