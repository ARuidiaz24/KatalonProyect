<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PATCH_UpdatePost</name>
   <tag></tag>
   <elementGuidId>480d4c82-5f8f-4ff7-af5a-9990a924349d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;${title}\&quot;,\n  \&quot;body\&quot;: \&quot;${content}\&quot;\n}&quot;,
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
      <webElementGuid>e8ddc5b6-5893-4cdc-8d2c-2f8d3e861f7c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/posts/${postId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>8c6e3973-dde0-4c7f-afd2-e44d1af5330a</id>
      <masked>false</masked>
      <name>postId</name>
   </variables>
   <variables>
      <defaultValue>'Titulo parcial'</defaultValue>
      <description></description>
      <id>79321784-689b-47cd-aeb3-cefde350bfce</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>'Body parcial'</defaultValue>
      <description></description>
      <id>06e1a317-bcf6-4bcb-ad49-f5c590af0107</id>
      <masked>false</masked>
      <name>content</name>
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
