<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST_createBooking</name>
   <tag></tag>
   <elementGuidId>cae79504-46ac-4ac9-9b19-89e3e81b65ae</elementGuidId>
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
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;${firstname}\&quot;,\n    \&quot;lastname\&quot; : \&quot;${lastname}\&quot;,\n    \&quot;totalprice\&quot; : ${totalprice},\n    \&quot;depositpaid\&quot; : ${depositpaid},\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;${checkin}\&quot;,\n        \&quot;checkout\&quot; : \&quot;${checkout}\&quot;\n    },\n    \&quot;additionalneeds\&quot; : \&quot;${additionalneeds}\&quot;\n}&quot;,
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
      <webElementGuid>5f44fe52-6317-4de2-ac33-f94ac9a22261</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>6e8e1009-25d2-408d-8b14-fa70f2b1622b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl_RB}/booking</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Ivan'</defaultValue>
      <description></description>
      <id>52f1ddc8-76d2-4694-b977-90d34f19873e</id>
      <masked>false</masked>
      <name>firstname</name>
   </variables>
   <variables>
      <defaultValue>'Villa'</defaultValue>
      <description></description>
      <id>6edb1968-ccd0-4162-a70f-2fe41f898342</id>
      <masked>false</masked>
      <name>lastname</name>
   </variables>
   <variables>
      <defaultValue>15000</defaultValue>
      <description></description>
      <id>27cad391-38d7-4f75-94da-0a60c6e99330</id>
      <masked>false</masked>
      <name>totalprice</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>b6868bea-fca0-46e7-a583-4214a4c062a0</id>
      <masked>false</masked>
      <name>depositpaid</name>
   </variables>
   <variables>
      <defaultValue>'2025-12-17'</defaultValue>
      <description></description>
      <id>9e7e4fc9-987f-43d3-b7f6-9751dbdd89b8</id>
      <masked>false</masked>
      <name>checkin</name>
   </variables>
   <variables>
      <defaultValue>'2025-12-18'</defaultValue>
      <description></description>
      <id>6d824c37-b232-4432-8ec0-9bc8cb2af00a</id>
      <masked>false</masked>
      <name>checkout</name>
   </variables>
   <variables>
      <defaultValue>'PruebaQA'</defaultValue>
      <description></description>
      <id>8dd846de-7dc4-4798-bff7-a225a101aee9</id>
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
