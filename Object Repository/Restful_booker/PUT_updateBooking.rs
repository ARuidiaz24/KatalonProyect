<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT_updateBooking</name>
   <tag></tag>
   <elementGuidId>7c5f03be-9e2c-4c25-b188-303ce1d002c1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationType>No Authorization</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;${firstname}\&quot;,\t\n    \&quot;lastname\&quot; : \&quot;${lastname}\&quot;,\n    \&quot;totalprice\&quot; : ${totalprice},\n    \&quot;depositpaid\&quot; : ${depositpaid},\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;${checkin}\&quot;,\n        \&quot;checkout\&quot; : \&quot;${checkout}\&quot;\n    },\n    \&quot;additionalneeds\&quot; : \&quot;${additionalneeds}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>93f39fad-97a1-4e8c-8614-a5e0bdb4e4fb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic YWRtaW46cGFzc3dvcmQxMjM=</value>
      <webElementGuid>5480c160-8f0c-439f-bec1-f2f1a951da84</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>202ce827-1e9f-4ea7-82db-abd7b622b96a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>token=${GlobalVaraible.token}</value>
      <webElementGuid>395d5a35-2381-4aad-962d-99c24d8a66d7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl_RB}/booking/${bookingId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>276212bf-fc25-4319-9d83-d2a38574f81b</id>
      <masked>false</masked>
      <name>bookingId</name>
   </variables>
   <variables>
      <defaultValue>'PruebaEditNameQA'</defaultValue>
      <description></description>
      <id>7b0dfdc0-1139-4568-a101-27f11f0ce7eb</id>
      <masked>false</masked>
      <name>firstname</name>
   </variables>
   <variables>
      <defaultValue>'PruebaEditLastnameQA'</defaultValue>
      <description></description>
      <id>24884dbb-682d-446d-83e0-4987dee9ed2d</id>
      <masked>false</masked>
      <name>lastname</name>
   </variables>
   <variables>
      <defaultValue>20000</defaultValue>
      <description></description>
      <id>311fdddd-15a7-434f-a882-8a3aca60197d</id>
      <masked>false</masked>
      <name>totalprice</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>60cf5d67-eae9-4ba1-83be-569ce21fddc4</id>
      <masked>false</masked>
      <name>depositpaid</name>
   </variables>
   <variables>
      <defaultValue>'2025-12-20'</defaultValue>
      <description></description>
      <id>54be6a75-3574-4677-9686-685574a03397</id>
      <masked>false</masked>
      <name>checkin</name>
   </variables>
   <variables>
      <defaultValue>'2025-12-21'</defaultValue>
      <description></description>
      <id>c29c2605-1d6b-4879-ac95-0202bf766f9d</id>
      <masked>false</masked>
      <name>checkout</name>
   </variables>
   <variables>
      <defaultValue>'PruebaEditAdditionalneedsQA'</defaultValue>
      <description></description>
      <id>1831292a-581b-4e1b-ba99-11858a190833</id>
      <masked>false</masked>
      <name>additionalneeds</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
