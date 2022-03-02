// Uses Declarative syntax to run commands inside a container.
pipeline {
    agent {
        kubernetes {
            yaml '''
apiVersion: v1
kind: Pod
spec:
  containers:
          - name: "shell"
            image: "docker:20"
            imagePullPolicy: IfNotPresent
            env:
            - name: DOCKER_HOST
              value: "ssh://root@94.130.8.239"
            command:
               - '/bin/sh'
               - '-c'
               - apk add git && mkdir ~/.ssh &&
                 echo LS0tLS1CRUdJTiBPUEVOU1NIIFBSSVZBVEUgS0VZLS0tLS0KYjNCbGJuTnphQzFyWlhrdGRqRUFBQUFBQkc1dmJtVUFBQUFFYm05dVpRQUFBQUFBQUFBQkFBQUJsd0FBQUFkemMyZ3RjbgpOaEFBQUFBd0VBQVFBQUFZRUF2RThwZVN0dGxKVkJLQVpvSXY1dWh2U3YvRTZJaldnWUhmNjVUbnB6My8zSndtUE15NXNxCmIzWkhqbStHUzVvZjUrbG1rbzhTRnQ3aHlXV2ZqL1d1OW84MDNKL1VVOVp3b1grbVFvMjhXejBRK0FBckg0V3phQm12Y1MKcDVjdjU5UTZXTjljampxNEpQT2JDbHNXcFNyL1ZyekdYbXFtODJDZ3ZaL1Q0YXN6RGtZQTFaV2tYTzRpKytqSE5Cb3J2aQoxdTltTzNlWkZoeThyVDV1TzBVdkN6WEdSU1J1S1ltSGptK0Y5MEZETnoyV1d1NnQwM2dpSUJpaUgrRWhKSmIvcERJSmgxCi9PWEpMcndzcVh0aXRKVDM0V09MZzFScHBhREZLbXFvMFJ1UnpKa1hTM3VmVVpzVjJBcU0xZzZFa1lLSXBMS3F2cUZEcWwKdStDV3VlTVVQbWtQR2d5MEVUVVJFOEs0Smx0V0FwTWhnQzgzWmlRSGZrOTgzZ3d1dlpIWTRLTXVlQjhPc3llVUpNMzl0bApjU2hpeFdhYU5wSjdLL1BxaW9veElCeUNEVXpZa3B2T3d6WGlpUlAxckp5bW9aOG4veFE4dHB1THdDanQzSFF1MEZkV2tSCitWWHJFNzNvWkVGUEUyaHM5TzlnTndwMy9Rb3lVZU9FcXdIaTlDbkxBQUFGaURiZVlmYzIzbUgzQUFBQUIzTnphQzF5YzIKRUFBQUdCQUx4UEtYa3JiWlNWUVNnR2FDTCtib2Iwci94T2lJMW9HQjMrdVU1NmM5Lzl5Y0pqek11YkttOTJSNDV2aGt1YQpIK2ZwWnBLUEVoYmU0Y2xsbjQvMXJ2YVBOTnlmMUZQV2NLRi9wa0tOdkZzOUVQZ0FLeCtGczJnWnIzRXFlWEwrZlVPbGpmClhJNDZ1Q1R6bXdwYkZxVXEvMWE4eGw1cXB2TmdvTDJmMCtHck13NUdBTldWcEZ6dUl2dm94elFhSzc0dGJ2Wmp0M21SWWMKdkswK2JqdEZMd3MxeGtVa2JpbUpoNDV2aGZkQlF6YzlsbHJ1cmRONElpQVlvaC9oSVNTVy82UXlDWWRmemx5UzY4TEtsNwpZclNVOStGamk0TlVhYVdneFNwcXFORWJrY3laRjB0N24xR2JGZGdLak5ZT2hKR0NpS1N5cXI2aFE2cGJ2Z2xybmpGRDVwCkR4b010QkUxRVJQQ3VDWmJWZ0tUSVlBdk4yWWtCMzVQZk40TUxyMlIyT0NqTG5nZkRyTW5sQ1ROL2JaWEVvWXNWbW1qYVMKZXl2ejZvcUtNU0FjZ2cxTTJKS2J6c00xNG9rVDlheWNwcUdmSi84VVBMYWJpOEFvN2R4MEx0QlhWcEVmbFY2eE85NkdSQgpUeE5vYlBUdllEY0tkLzBLTWxIamhLc0I0dlFweXdBQUFBTUJBQUVBQUFHQkFKaDhEZHhsczBWbkd5emJDMGFTKzFOakhvCkxUNFRXSWZrY0R0bkI3TGd2S2lhOVVlMGpBYkR0Mzd2ZkREVjk0L2E4Z3pBT1B3Umt1QUczQ1VkUVVJVXJjcWZTaHBDRUQKZTQ3N085bVd4bGluZVc4cFM4SXAxUjVOOWtoUUdhcHBzMnVzeXVpbGRONjBGWTFrb3Jlb25pNklNSE9DYktmRWlHRFlPZgp3Y1p5aUZSZGtWMWk0SWF6K3pZN3g5dEQrZm4wOW02a1RyVXRhYURKckkvQ0FZcE1DU0dleHhWSmRrRzVYV2J3ZktIRERHCk9WT0hCTkZCN1hoS0dqWndhaE1rQlpmOEhES0JkbDcwYmJERWUrQTNwdFpUQjFkQWZWbExTblNLV3hQRk5UcGVieUZiSk4KL2h0VGFCZFM3M1YrV0RoWmJ0cTlLSENwL0FzekpZYW9xOXZ2MmdLNVQ3SHZMQTFLVStNelhjQ2M1SE44bG95MWE3Q1dCWQpMeWZvTHBOOHQ2K0ZWQ0xQWXpja0tteW96NVFtZ1hpUmptTUVlMlJEOVFYY2d1QTc1SjhUcjBORnZKL0gzbTRDUUU3YVpSCnBvT3pZSE95Unp1VEdlV1BsN3JwcDdCSk1OYUlLbUUwWjRDVFYvUzhCbVhYSlArMzFjTVBlU0R5QTVleUt6cnpjK1VRQUEKQU1FQWt4L3FKOTFIYjVSaW9ZaWcydmQrc2hrOTEvRFdXZUsvaFhNb3FTUWhVTmFSSWFDTWZtNE8zdnFPanVuTUNWbHF3WgprbDFrdlJ1a2NESUJPVWEyL2tsck1qZ0hiSTJjRER4cmRjVlhObDJKVkpKTDIwbDlrVHEwd3hUaURqQUdpVmNMRTNxY1NICkIzMFBTSy9md2lTTFBkUHRlNGRUU3BXSnB2UGVpNkRzQUZWeUpxa09KTXF0MVRBYlI0TnlGUWdNSkZOcm53SW9DYTRVSHMKVDdXOXRPTTRRSmQzVWpPNk1seHAxRDNxS0dVZUVERVBnODlhd3JsUmR0Q043clFlNTRBQUFBd1FEdE9lMWs2YnlZNFhkUQo5WjUyZ0xSbWt3OFVFRm5zSEFXN0RZcW50WnRqNlVZckR2WTUwV21PUlF2RXpLM1J4VTJJeWxpYVo1UVF1QlZrSTFlS2U3CldkMk5JTkR0WXBiTUNmc0tZR0pxTWhhaGx5QlRDQ0IxTm5zbHZQTEoyaVZKeElFKzdtYkpYc0lPNUxrSGcrd1VCK0Y5dUoKc1ZkRU1iSVpHaXRwT2FmSTlUUEpUdFZJR1VSRURWRUo1U2VCOU4zMkp4c3hCcWRROHc0WUVidkZWRFFKdTQ2cGE5VnFBRgpDNWhsNnNVSmxETzl3MGdBREV5WDVUajY2bnYxdlQ4d2tBQUFEQkFNczJNODZ3MitUUjV2VmVrcEJIN0Q4dlJ6YUdOa2FMCkpKTjRLR21xclZVTnBlUld1VFFMVk9nRGpsMFlRcHdNV0tESklpajducUpDS2pjdUF0UGlHYzY2T3Njb1NlSm1hbnVjY1UKTThZb1YyWmFHZElHZ3htYUhXdHV3SnljVUdYNkYrVGJlTVB6dzlneUwzd2NCdmZKWFhEcStzWWJpRVBrN2JPREp5Uk1sRApBMzN6Rk02a2k5aUlJbkhlNlQ1aVRxRCttWEExVmk4bmpjSlA2bWhQbE1uZjFCTGlZWmEzNnBBdWpET3kxdEpUL1I5RXo4Cndva2crTHdNUnhuVW1ITXdBQUFBMXliMjkwUUVoUUxWcENiMjlyQVFJREJBPT0KLS0tLS1FTkQgT1BFTlNTSCBQUklWQVRFIEtFWS0tLS0tCg==
                 | base64 -d > ~/.ssh/id_rsa && chmod 600 ~/.ssh/id_rsa &&
                 echo SG9zdCAqCiAgU3RyaWN0SG9zdEtleUNoZWNraW5nPW5vCiAgU2VydmVyQWxpdmVJbnRlcnZhbD0xMAogIFRDUEtlZXBBbGl2ZT15ZXMKICBJZGVudGl0aWVzT25seT15ZXMK
                 | base64 -d > ~/.ssh/config &&
                 tail -f /dev/null
            resources:
              requests:
                cpu: 500m
                memory: "256Mi"
'''
            defaultContainer 'shell'
        }
    }
    stages {
        stage('BUILD IMAGE') {
            steps {
              script {
                git branch: 'stable', credentialsId: 'github-token', 
                    url: 'https://github.com/RealisNetwork/Realis.Network.git'
                withCredentials([string(credentialsId: 'dockerhub-token', variable: 'dockerhub')]) {
                   sh '''
                      env
                      docker build . -t realisnetwrok/blockchain:${GIT_COMMIT}
                      echo $dockerhub | docker login -u realisnetwrok --password-stdin
                      docker push realisnetwrok/blockchain:${GIT_COMMIT}
                     '''
                }   
              }    
            }
        }
    }
}

