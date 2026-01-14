<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST_CreatePost</name>
   <tag></tag>
   <elementGuidId>31aace29-5a4b-4e0d-a1ca-66cc7a98362e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;${title}\&quot;,\n  \&quot;body\&quot;: \&quot;${content}\&quot;,\n  \&quot;userId\&quot;: ${userId}\n}&quot;,
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
      <webElementGuid>ca1c5ce3-9136-4a3a-a4c3-de640b0f9d4a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/posts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Prueba'</defaultValue>
      <description></description>
      <id>af87ded7-b738-4cbf-b36a-4d5922294c3b</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>'Contenido de prueba'</defaultValue>
      <description></description>
      <id>ccc6e62d-005b-4617-b5d4-7b8fb78e5c97</id>
      <masked>false</masked>
      <name>content</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>ef182f7b-a739-429c-9e4b-f91ec4e115a8</id>
      <masked>false</masked>
      <name>userId</name>
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
