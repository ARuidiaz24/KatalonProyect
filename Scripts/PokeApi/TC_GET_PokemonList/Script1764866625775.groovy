import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.google.api.client.json.Json
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys


import com.kms.katalon.core.testobject.ObjectRepository as OR
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
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